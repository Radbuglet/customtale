package net.coopfury.customtale_protocol_tools

import java.lang.reflect.Constructor
import java.lang.reflect.Modifier
import java.lang.reflect.Parameter
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

        println(packet.name)

        val packetInstance = randomizeInstance(packet, Random.Default, 0)
        println(packetInstance.toString())
    }
}

fun randomLenForDepth(rng: Random, depth: Int) : Int {
    if (depth > 8)
        return 0

    return rng.nextInt(8)
}

fun randomizeInstance(ty: Parameter, rng: Random, depth: Int) : Any? {
    var isNullable = false
    for (anno in ty.annotations)
        if (anno.annotationClass.simpleName == "Nullable")
            isNullable = true

    if (isNullable && rng.nextBoolean())
        return null

    return randomizeInstance(ty.parameterizedType, rng, depth)
}

fun randomizeInstance(ty: Type, rng: Random, depth: Int) : Any? {
    if (ty is ParameterizedType)
        return randomizeInstance(ty, rng, depth)

    return randomizeInstance(ty as Class<*>, rng, depth)
}

fun randomizeInstance(ty: ParameterizedType, rng: Random, depth: Int) : Any? {
    val args = ty.actualTypeArguments
    if (args.isEmpty())
        return randomizeInstance(ty.rawType, rng, depth)

    if (ty.rawType == Map::class.java) {
        val len = randomLenForDepth(rng, depth)
        val map = mutableMapOf<Any?, Any?>()

        (0..<len).forEach { _ ->
            map[randomizeInstance(args[0], rng, depth + 1)] = randomizeInstance(args[0], rng, depth + 1)
        }

        return map
    }

    throw UnsupportedOperationException("unknown parameterized class type $ty")
}

@Suppress("PLATFORM_CLASS_MAPPED_TO_KOTLIN")
fun randomizeInstance(ty: Class<*>, rng: Random, depth: Int) : Any? {
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

    if (ty == Double::class.java)
        return rng.nextDouble()

    if (ty == String::class.java)
        return rng.nextInt().toString()

    if (ty == UUID::class.java)
        return UUID.randomUUID()

    if (ty.isArray) {
        val len = randomLenForDepth(rng, depth)
        val arr = java.lang.reflect.Array.newInstance(ty.componentType, len)
        (0..<len).forEach { i ->
            java.lang.reflect.Array.set(arr, i, randomizeInstance(ty.componentType, rng, depth + 1))
        }

        return arr
    }

    if (ty.name.startsWith("com.hypixel.hytale.protocol")) {
        if (ty.isEnum) {
            val variants = ty.getField("VALUES").get(null) as Array<*>
            return variants[rng.nextInt(variants.size)]
        } else if (Modifier.isAbstract(ty.modifiers)) {
            // TODO
            return null
        } else {
            return randomizeInstanceHytaleStruct(ty, rng, depth)
        }
    }

    throw UnsupportedOperationException("unknown unparameterized class type $ty")
}

fun randomizeInstanceHytaleStruct(ty: Class<*>, rng: Random, depth: Int) : Any? {
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
            args += randomizeInstance(param, rng, depth + 1)
        }

        return ctor.newInstance(*args.toTypedArray())
    }

    if (emptyCtor != null) {
        return emptyCtor.newInstance()
    }

    throw UnsupportedOperationException("missing constructor for $ty")
}
