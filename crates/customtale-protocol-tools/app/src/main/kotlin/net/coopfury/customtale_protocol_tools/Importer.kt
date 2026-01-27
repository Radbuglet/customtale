package net.coopfury.customtale_protocol_tools

import java.lang.reflect.Constructor
import java.lang.reflect.Modifier
import java.lang.reflect.Parameter
import java.lang.reflect.ParameterizedType
import java.lang.reflect.Type
import java.util.UUID

private fun isNullable(ty: Parameter) : Boolean {
    var isNullable = false
    for (anno in ty.annotations)
        if (anno.annotationClass.simpleName == "Nullable")
            isNullable = true

    return isNullable
}

class Importer {
    val importedStructs = mutableMapOf<Class<*>, CodecNode.Struct>()

    fun import(ty: Parameter) : CodecNode {
        val inner = import(ty.parameterizedType)

        return if (isNullable(ty)) {
            CodecNode.Optional(inner)
        } else {
            inner
        }
    }

    fun import(ty: Type) : CodecNode {
        return if (ty is ParameterizedType) {
            import(ty)
        } else {
            import(ty as Class<*>)
        }
    }

    fun import(ty: ParameterizedType) : CodecNode {
        val args = ty.actualTypeArguments
        if (args.isEmpty())
            return import(ty.rawType)

        if (ty.rawType == Map::class.java) {
            return CodecNode.VarMap(import(args[0]), import(args[1]), DEFAULT_MAX_VAR_LEN)
        }

        throw UnsupportedOperationException("unknown parameterized class type $ty")
    }

    @Suppress("PLATFORM_CLASS_MAPPED_TO_KOTLIN")
    fun import(ty: Class<*>) : CodecNode {
        if (ty == Boolean::class.java || ty == java.lang.Boolean::class.java)
            return CodecNode.LeBool()

        if (ty == Byte::class.java || ty == java.lang.Byte::class.java)
            return CodecNode.LeByte()

        if (ty == Short::class.java || ty == java.lang.Short::class.java)
            return CodecNode.LeShort()

        if (ty == Int::class.java || ty == Integer::class.java)
            return CodecNode.LeInt()

        if (ty == Long::class.java || ty == java.lang.Long::class.java)
            return CodecNode.LeLong()

        if (ty == Float::class.java || ty == java.lang.Float::class.java)
            return CodecNode.LeFloat()

        if (ty == Double::class.java  || ty == java.lang.Double::class.java)
            return CodecNode.LeDouble()

        if (ty == String::class.java)
            return CodecNode.VarString(DEFAULT_MAX_VAR_LEN)

        if (ty == UUID::class.java)
            return CodecNode.Uuid()

        if (ty.isArray) {
            return CodecNode.VarList(import(ty.componentType), DEFAULT_MAX_VAR_LEN)
        }

        if (ty.name.startsWith("com.hypixel.hytale.protocol")) {
            return if (ty.isEnum) {
                CodecNode.Enum(ty)
            } else if (Modifier.isAbstract(ty.modifiers)) {
                // TODO: Figure out how to port these automatically.
                CodecNode.Tainted()
            } else {
                importHytaleStruct(ty)
            }
        }

        throw UnsupportedOperationException("unknown unparameterized class type $ty")
    }

    fun importHytaleStruct(ty: Class<*>) : CodecNode {
        var codec = importedStructs[ty]
        if (codec != null) {
            return codec
        }

        codec = CodecNode.Struct()
        importedStructs[ty] = codec

        var emptyCtor = null as Constructor<*>?

        for (ctor in ty.constructors) {
            val params = ctor.parameters

            if (params.size == 0) {
                emptyCtor = ctor
                continue
            }

            if (params.size == 1 && params[0].type == ty)
                continue

            val paramCodecs = params.map { param -> CodecNode.StructField(param.name, import(param)) }
            codec.init(ctor, paramCodecs, OptionSerdeMode.Variable)
            return codec
        }

        if (emptyCtor != null) {
            codec.init(emptyCtor, emptyList(), OptionSerdeMode.Variable)
            return codec
        }

        throw UnsupportedOperationException("missing constructor for $ty")
    }
}
