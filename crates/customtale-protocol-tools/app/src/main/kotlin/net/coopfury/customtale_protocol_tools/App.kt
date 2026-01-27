package net.coopfury.customtale_protocol_tools

import io.netty.buffer.Unpooled
import io.netty.buffer.ByteBuf
import java.net.URL
import java.net.URLClassLoader
import java.nio.file.Path
import kotlin.random.Random

fun main(args: Array<String>) {
    if (args.size != 1)
        throw IllegalArgumentException("bad usage")

    val loader = URLClassLoader(
        arrayOf<URL>(Path.of(args[0]).toUri().toURL()),
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
    val importer = Importer()

    for (packet in packets.values) {
        val packetTy = typeField.get(packet) as Class<*>
        val packetId = packetTy.getField("PACKET_ID").get(null) as Int

        val packetCodec = importer.import(packetTy)
        if (packetCodec.isTainted()) {
            continue
        }

        println("#[test]")
        println("fn roundtrip_${packetTy.simpleName}() {")
        (0..<100).forEach { _ ->
            val packetInstance = packetCodec.generateInstance(rng)
            val outBuf = Unpooled.buffer()
            serializeMethod.invoke(packetInstance, outBuf)

            val outBufRaw = ByteArray(outBuf.readableBytes())
            outBuf.readBytes(outBufRaw)

            println("    check_round_trip($packetId, ${formatByteArray(outBufRaw)});")
        }
        println("}")
        println()
    }
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
