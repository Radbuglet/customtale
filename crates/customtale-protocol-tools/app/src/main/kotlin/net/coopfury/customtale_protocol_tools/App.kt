package net.coopfury.customtale_protocol_tools

import io.netty.buffer.Unpooled
import io.netty.buffer.ByteBuf
import java.lang.reflect.Modifier
import java.net.URL
import java.net.URLClassLoader
import java.nio.file.Path
import java.lang.reflect.Array as ArrayReflect
import kotlin.io.path.writeText
import kotlin.random.Random

fun main(args: Array<String>) {
    if (args.size != 2)
        throw IllegalArgumentException("bad usage")

    val jarPath = Path.of(args[0])
    val outPath = Path.of(args[1])

    val loader = URLClassLoader(
        arrayOf<URL>(jarPath.toUri().toURL()),
        ClassLoader.getSystemClassLoader())

    val serializeMethod = loader
        .loadClass("$PACKET_PKG_ROOT.Packet")
        .getMethod("serialize", ByteBuf::class.java)

    val packetRegistry = loader.loadClass("$PACKET_PKG_ROOT.PacketRegistry")
    val packets = packetRegistry.getMethod("all").invoke(null) as Map<*, *>

    val typeField = loader
        .loadClass($$"$$PACKET_PKG_ROOT.PacketRegistry$PacketInfo")
        .getDeclaredField("type")

    typeField.isAccessible = true

    val rng = Random(4)
    val importer = Importer(loader)

    // Generate tests
    val testSb = StringBuilder()
    testSb.append(getResourceAsText("/prefix_tests.rs"))

    for (packet in packets.values) {
        val packetTy = typeField.get(packet) as Class<*>
        val packetId = packetTy.getField("PACKET_ID").get(null) as Int

        val packetCodec = importer.import(packetTy)
        if (packetCodec.isTainted()) {
            continue
        }

        testSb.append("#[test]\n")
        testSb.append("fn roundtrip_${packetTy.simpleName}() {\n")
        for (i in 0..<10) {
            if (i != 0)
                testSb.append("\n")

            val packetInstance = packetCodec.generateInstance(rng)
            val outBuf = Unpooled.buffer()
            serializeMethod.invoke(packetInstance, outBuf)

            val outBufRaw = ByteArray(outBuf.readableBytes())
            outBuf.readBytes(outBufRaw)

            testSb.append("    // ")
            testSb.append(debugPrintInstance(packetInstance))
            testSb.append("\n")
            testSb.append("    check_round_trip($packetId, ${formatByteArray(outBufRaw)});\n")
        }
        testSb.append("}\n\n")
    }

    outPath.resolve("tests.rs").writeText(testSb.toString())

    // Generate definitions
    val defSb = StringBuilder()
    defSb.append(getResourceAsText("/prefix_defs.rs"))

    defSb.append("define_packets! {")
    for (def in importer.definitions) {
        if (def.packet == null || def.codec.isTainted())
            continue

        defSb.append("    ")
        def.codec.toRustType(defSb)
        defSb.append(",\n")
    }
    defSb.append("}\n\n")

    for (def in importer.definitions) {
        def.toRustDefinition(defSb)
    }

    outPath.resolve("packets.rs").writeText(defSb.toString())
}

private fun formatByteArray(arr: ByteArray) : String {
    val builder = StringBuilder()

    builder.append("b\"")

    val fmt = HexFormat {
        upperCase = true
        number {
            removeLeadingZeros = false
            prefix = "\\x"
        }
        bytes {
            bytesPerGroup = 2
            groupSeparator = "."
        }
    }

    for (elem in arr) {
        if ((33..126).contains(elem) && elem != 34.toByte() && elem != 92.toByte())
            builder.append(elem.toInt().toChar())
        else
            builder.append(elem.toHexString(fmt))
    }

    builder.append("\"")

    return builder.toString()
}

private fun debugPrintInstance(target: Any?) : String {
    val sb = StringBuilder()
    val reentrancyMap = mutableSetOf<HashById<Any?>>()

    var printInner: Function1<Any?, Unit>? = null

    fun printInnerNoGuard(target: Any) {
        val clazz = target.javaClass

        if (clazz.isEnum || clazz.isPrimitive) {
            sb.append(target.toString())
            return
        }

        if (clazz.isArray) {
            sb.append("[")
            for (i in 0..<ArrayReflect.getLength(target)) {
                if (i > 0) {
                    sb.append(", ")
                }

                printInner!!.invoke(ArrayReflect.get(target, i))
            }
            sb.append("]")
            return
        }

        if (target is Map<*, *>) {
            var isSubsequent = false

            sb.append("Map {")

            for (elem in target) {
                if (isSubsequent)
                    sb.append(", ")
                isSubsequent = true

                printInner!!.invoke(elem.key)
                sb.append(": ")
                printInner!!.invoke(elem.value)
            }

            sb.append("}")
            return
        }

        if (target is Collection<*>) {
            var isSubsequent = false

            sb.append(clazz.simpleName)
            sb.append(" [")

            for (elem in target) {
                if (isSubsequent)
                    sb.append(", ")
                isSubsequent = true

                printInner!!.invoke(elem)
            }

            sb.append("]")
            return
        }

        if (!clazz.name.startsWith(PACKET_PKG_ROOT)) {
            sb.append(target.toString())
            return
        }

        sb.append(clazz.simpleName)
        sb.append(" {")

        var isSubsequent = false

        for (field in clazz.fields) {
            if (Modifier.isStatic(field.modifiers))
                continue

            if (isSubsequent)
                sb.append(", ")
            isSubsequent = true

            sb.append(field.name)
            sb.append(": ")
            printInner!!.invoke(field.get(target))
        }

        sb.append("}")
    }

    printInner = fun(target: Any?) {
        if (target == null) {
            sb.append("null")
            return
        }

        if (!reentrancyMap.add(HashById(target))) {
            sb.append("(...)")
            return
        }

        printInnerNoGuard(target)
        reentrancyMap.remove(target)
    }

    printInner.invoke(target)

    return sb.toString()
}

private class HashById<T>(val inner: T) {
    override fun equals(other: Any?): Boolean {
        return other is HashById<*> && inner === other.inner
    }

    override fun hashCode(): Int {
        return System.identityHashCode(inner)
    }
}

// https://stackoverflow.com/a/53018129
private fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()
