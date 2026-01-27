package net.coopfury.customtale_protocol_tools

import io.netty.buffer.Unpooled
import io.netty.buffer.ByteBuf
import java.net.URL
import java.net.URLClassLoader
import java.nio.file.Path
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
        .loadClass("com.hypixel.hytale.protocol.Packet")
        .getMethod("serialize", ByteBuf::class.java)

    val packetRegistry = loader.loadClass("com.hypixel.hytale.protocol.PacketRegistry")
    val packets = packetRegistry.getMethod("all").invoke(null) as Map<*, *>

    val typeField = loader
        .loadClass($$"com.hypixel.hytale.protocol.PacketRegistry$PacketInfo")
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
        (0..<100).forEach { _ ->
            val packetInstance = packetCodec.generateInstance(rng)
            val outBuf = Unpooled.buffer()
            serializeMethod.invoke(packetInstance, outBuf)

            val outBufRaw = ByteArray(outBuf.readableBytes())
            outBuf.readBytes(outBufRaw)

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
    defSb.append("}")

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

// https://stackoverflow.com/a/53018129
private fun getResourceAsText(path: String): String? =
    object {}.javaClass.getResource(path)?.readText()