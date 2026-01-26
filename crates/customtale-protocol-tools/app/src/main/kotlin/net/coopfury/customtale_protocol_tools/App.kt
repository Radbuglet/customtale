package net.coopfury.customtale_protocol_tools

import io.netty.buffer.Unpooled
import java.lang.reflect.*
import java.net.URL
import java.net.URLClassLoader
import java.nio.file.Path
import java.util.*
import kotlin.random.Random

fun main(args: Array<String>) {
    if (args.size != 1)
        throw IllegalArgumentException("bad usage")

    val loader = URLClassLoader(
        arrayOf<URL>(Path.of(args[0]).toUri().toURL()),
        ClassLoader.getSystemClassLoader())

    val serializeMethod = loader.loadClass("com.hypixel.hytale.protocol.Packet").methods[1] // TODO
    val packetRegistry = loader.loadClass("com.hypixel.hytale.protocol.PacketRegistry")
    val packets = packetRegistry.getMethod("all").invoke(null) as Map<*, *>

    val typeField = loader.loadClass($$"com.hypixel.hytale.protocol.PacketRegistry$PacketInfo").getDeclaredField("type")
    typeField.isAccessible = true

    for (packet in packets.values) {
        val packet = typeField.get(packet) as Class<*>

        println(packet.name)

        val packetInstance = randomizeInstance(packet, Random.Default) ?: continue
        val outBuf = Unpooled.buffer()
        serializeMethod.invoke(packetInstance, outBuf)

        val outBufRaw = ByteArray(outBuf.readableBytes())
        outBuf.readBytes(outBufRaw)

        println(formatByteArray(outBufRaw))
    }
}

fun randomizeInstance(ty: Class<*>, rng: Random) : Any? {
    val gen = RandomInstanceGenerator(rng)
    val instance = gen.randomize(ty, 0)

    if (gen.tainted)
        return null

    return instance
}

fun formatByteArray(arr: ByteArray) : String {
    val builder = StringBuilder()
    var first = true

    builder.append("[")

    for (elem in arr) {
        if (!first) {
            builder.append(", ")
        }
        builder.append(elem.toUByte())
        first = false
    }

    builder.append("]")

    return builder.toString()
}

class RandomInstanceGenerator(val rng: Random) {
    var tainted: Boolean = false

    fun randomLenForDepth(depth: Int) : Int {
        if (depth > 8)
            return 0

        return rng.nextInt(8)
    }

    fun randomize(ty: Parameter, depth: Int) : Any? {
        var isNullable = false
        for (anno in ty.annotations)
            if (anno.annotationClass.simpleName == "Nullable")
                isNullable = true

        if (isNullable && rng.nextBoolean())
            return null

        return randomize(ty.parameterizedType, depth)
    }

    fun randomize(ty: Type, depth: Int) : Any? {
        if (ty is ParameterizedType)
            return randomize(ty, depth)

        return randomize(ty as Class<*>, depth)
    }

    fun randomize(ty: ParameterizedType, depth: Int) : Any? {
        val args = ty.actualTypeArguments
        if (args.isEmpty())
            return randomize(ty.rawType, depth)

        if (ty.rawType == Map::class.java) {
            val len = randomLenForDepth(depth)
            val map = mutableMapOf<Any?, Any?>()

            (0..<len).forEach { _ ->
                map[randomize(args[0], depth + 1)] = randomize(args[1], depth + 1)
            }

            return map
        }

        throw UnsupportedOperationException("unknown parameterized class type $ty")
    }

    @Suppress("PLATFORM_CLASS_MAPPED_TO_KOTLIN")
    fun randomize(ty: Class<*>, depth: Int) : Any? {
        if (ty == Boolean::class.java || ty == java.lang.Boolean::class.java)
            return rng.nextBoolean()

        if (ty == Byte::class.java || ty == java.lang.Byte::class.java)
            return rng.nextInt().toByte()

        if (ty == Short::class.java || ty == java.lang.Short::class.java)
            return rng.nextInt().toShort()

        if (ty == Int::class.java || ty == Integer::class.java)
            return rng.nextInt()

        if (ty == Long::class.java || ty == java.lang.Long::class.java)
            return rng.nextLong()

        if (ty == Float::class.java || ty == java.lang.Float::class.java)
            return rng.nextFloat()

        if (ty == Double::class.java  || ty == java.lang.Double::class.java)
            return rng.nextDouble()

        if (ty == String::class.java)
            return rng.nextInt().toString()

        if (ty == UUID::class.java)
            return UUID.randomUUID()

        if (ty.isArray) {
            val len = randomLenForDepth(depth)
            val arr = java.lang.reflect.Array.newInstance(ty.componentType, len)
            (0..<len).forEach { i ->
                java.lang.reflect.Array.set(arr, i, randomize(ty.componentType, depth + 1))
            }

            return arr
        }

        if (ty.name.startsWith("com.hypixel.hytale.protocol")) {
            if (ty.isEnum) {
                val variants = ty.getField("VALUES").get(null) as Array<*>
                return variants[rng.nextInt(variants.size)]
            } else if (Modifier.isAbstract(ty.modifiers)) {
                tainted = true
                return null
            } else {
                return randomizeInstanceHytaleStruct(ty, depth)
            }
        }

        throw UnsupportedOperationException("unknown unparameterized class type $ty")
    }

    fun randomizeInstanceHytaleStruct(ty: Class<*>, depth: Int) : Any? {
        var emptyCtor = null as Constructor<*>?

        for (ctor in ty.constructors) {
            val params = ctor.parameters

            if (params.size == 0) {
                emptyCtor = ctor
                continue
            }

            if (params.size == 1 && params[0].type == ty)
                continue

            val args = mutableListOf<Any?>()

            for (param in params) {
                args += randomize(param, depth + 1)
            }

            return ctor.newInstance(*args.toTypedArray())
        }

        if (emptyCtor != null) {
            return emptyCtor.newInstance()
        }

        throw UnsupportedOperationException("missing constructor for $ty")
    }

}
