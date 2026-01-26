package net.coopfury.customtale_protocol_tools

import java.lang.reflect.ParameterizedType
import java.lang.reflect.Type
import java.net.URL
import java.net.URLClassLoader
import java.nio.file.Path
import java.util.UUID
import kotlin.random.Random

fun main(args: Array<String>) {
    if (args.size != 1)
        throw IllegalArgumentException("bad usage")

    val loader = URLClassLoader(
        arrayOf<URL>(Path.of(args[0]).toUri().toURL()),
        ClassLoader.getSystemClassLoader())

    val packetRegistry = loader.loadClass("com.hypixel.hytale.protocol.PacketRegistry")
    val packets = packetRegistry.getMethod("all").invoke(null) as Map<*, *>

    val typeField = loader.loadClass($$"com.hypixel.hytale.protocol.PacketRegistry$PacketInfo").getDeclaredField("type")
    typeField.isAccessible = true

    for (packet in packets.values) {
        val packet = typeField.get(packet) as Class<*>

        if (packet.name.startsWith("com.hypixel.hytale.protocol.packets.connect") || packet.name.startsWith("com.hypixel.hytale.protocol.packets.auth"))
            continue

        println(packet.name)
        randomizeInstance(packet, Random.Default)
    }
}

fun randomizeInstance(ty: Type, rng: Random) : Any? {
    if (ty is ParameterizedType)
        return randomizeInstance(ty, rng)

    return randomizeInstance(ty as Class<*>, rng)
}

fun randomizeInstance(ty: ParameterizedType, rng: Random) : Any? {
    val args = ty.actualTypeArguments
    if (args.isEmpty())
        return randomizeInstance(ty.rawType, rng)

    if (ty.rawType == Map::class.java) {
        val len = rng.nextInt(8)
        val map = mutableMapOf<Any?, Any?>()

        (0..<len).forEach { _ ->
            map[randomizeInstance(args[0], rng)] = randomizeInstance(args[0], rng)
        }

        return map
    }

    throw UnsupportedOperationException("unknown parameterized class type $ty")
}

@Suppress("PLATFORM_CLASS_MAPPED_TO_KOTLIN")
fun randomizeInstance(ty: Class<*>, rng: Random) : Any? {
    if (ty == Boolean::class.java || ty == java.lang.Boolean::class.java)
        return rng.nextBoolean()

    if (ty == Byte::class.java || ty == java.lang.Byte::class.java)
        return rng.nextInt().toByte()

    if (ty == Short::class.java || ty == java.lang.Short::class.java)
        return rng.nextInt().toShort()

    if (ty == Int::class.java || ty == Integer::class.java)
        return rng.nextInt()

    if (ty == Long::class.java)
        return rng.nextLong()

    if (ty == Float::class.java)
        return rng.nextFloat()

    if (ty == String::class.java)
        return rng.nextInt().toString()

    if (ty == UUID::class.java)
        return UUID.randomUUID()

    if (ty.isArray) {
        val len = rng.nextInt(8)
        val arr = java.lang.reflect.Array.newInstance(ty.componentType, len)
        (0..<len).forEach { i ->
            java.lang.reflect.Array.set(arr, i, randomizeInstance(ty.componentType, rng))
        }

        return arr
    }

    if (ty.name.startsWith("com.hypixel.hytale.protocol"))
        return randomizeInstanceHytale(ty, rng)

    throw UnsupportedOperationException("unknown unparameterized class type $ty")
}

fun randomizeInstanceHytale(ty: Class<*>, rng: Random) : Any? {
    for (ctor in ty.constructors) {
        val params = ctor.parameters

        if (params.size == 0)
            continue

        if (params.size == 1 && params[0].type == ty)
            continue

        val args = mutableListOf<Any?>()

        for (param in params) {
            args += randomizeInstance(param.parameterizedType, rng)
        }

        ctor.newInstance(*args.toTypedArray())
    }

    return null
}
