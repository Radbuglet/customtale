package net.coopfury.customtale_protocol_tools

import com.google.common.reflect.ClassPath
import java.lang.reflect.AnnotatedElement
import java.lang.reflect.Field
import java.lang.reflect.Modifier
import java.lang.reflect.ParameterizedType
import java.lang.reflect.Type
import java.util.UUID

private fun isNullable(ty: AnnotatedElement) : Boolean {
    var isNullable = false
    for (anno in ty.annotations)
        if (anno.annotationClass.simpleName == "Nullable")
            isNullable = true

    return isNullable
}

class Importer(val loader: ClassLoader) {
    private val importedAggregates = mutableMapOf<Class<*>, CodecNode>()
    private val importedEnums = mutableMapOf<Class<*>, CodecNode.Enum>()
    private val definitionsMut = mutableListOf<ImportedDefinition>()

    val definitions: List<ImportedDefinition> get() = definitionsMut

    private val directSubclassMap = run {
        val map = mutableMapOf<Class<*>, MutableList<Class<*>>>()

        for (clazzLazy in ClassPath.from(loader).allClasses) {
            if (!clazzLazy.packageName.startsWith(PACKET_PKG_ROOT))
                continue

            val clazz = clazzLazy.load()
            val superClass = clazz.superclass

            if (superClass == null || superClass == Any::class.java)
                continue

            var list = map[superClass]
            if (list == null) {
                list = mutableListOf()
                map[superClass] = list
            }

            list += clazz
        }

        map
    }

    fun import(name: String) : CodecNode {
        return import(loader.loadClass(name))
    }

    fun import(ty: Field) : CodecNode {
        val special = overrideSpecialField(ty, this)
        if (special != null)
            return special

        val inner = import(ty.genericType)

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

        if (ty.name.startsWith(PACKET_PKG_ROOT)) {
            return if (ty.isEnum) {
                importHytaleEnum(ty)
            } else {
                importHytaleAggregate(ty)
            }
        }

        throw UnsupportedOperationException("unknown unparameterized class type $ty")
    }

    fun importHytaleEnum(ty: Class<*>) : CodecNode {
        var codec = importedEnums[ty]

        if (codec == null) {
            codec = CodecNode.Enum(ty)
            importedEnums[ty] = codec
            definitionsMut += ImportedDefinition(packet = null, codec = codec)
        }

        return codec
    }

    fun importHytaleAggregate(ty: Class<*>) : CodecNode {
        // See whether we've imported the aggregate before
        val codecCached = importedAggregates[ty]
        if (codecCached != null) {
            return codecCached
        }

        // If not, determine the packet information
        val packetAnnotation = if (ty.fields.any { f -> f.name == "PACKET_ID" }) {
            PacketAnnotation(
                id = ty.getField("PACKET_ID").get(null) as Int,
                maxSize = ty.getField("MAX_SIZE").get(null) as Int,
                compressed = ty.getField("IS_COMPRESSED").get(null) as Boolean,
                categories = getPacketCategory(ty),
            )
        } else {
            null
        }

        // If it's a union, generate it.
        if (Modifier.isAbstract(ty.modifiers)) {
            val codec = CodecNode.Union(ty)
            definitionsMut += ImportedDefinition(packetAnnotation, codec)
            importedAggregates[ty] = codec

            val getTypeIdMethod = ty.getMethod("getTypeId")

            val variants = getAllSubtypes(ty)
                .filter { ty -> !Modifier.isAbstract(ty.modifiers) }
                .sortedBy { ty ->
                    val instance = ty.getConstructor().newInstance()
                    getTypeIdMethod.invoke(instance) as Int
                }
                .map { ty -> CodecNode.StructField(ty.simpleName, import(ty)) }

            codec.init(variants)
            return codec
        }

        // Otherwise, find the struct's data constructor and import all fields
        val codec = CodecNode.Struct(if (isStructSmall(ty)) OptionSerdeMode.Fixed else OptionSerdeMode.Variable)
        definitionsMut += ImportedDefinition(packetAnnotation, codec)
        importedAggregates[ty] = codec

        val fields = getAllFields(ty).filter { field -> !Modifier.isStatic(field.modifiers) }

        for (ctor in ty.constructors) {
            val params = ctor.parameters

            if (params.size != fields.size) {
                continue
            }

            if (params.map { p -> p.parameterizedType } != fields.map { f -> f.genericType })
                continue

            val paramCodecs = fields.map { field -> CodecNode.StructField(field.name, import(field)) }
            codec.init(ctor, paramCodecs)
            return codec
        }

        throw UnsupportedOperationException("missing constructor for $ty")
    }

    private fun getAllFields(ty: Class<*>) : List<Field> {
        val superClasses = mutableListOf<Class<*>>()
        var curr = ty as Class<*>?

        while (curr != null) {
            superClasses += curr
            curr = curr.superclass
        }

        val fields = mutableListOf<Field>()
        for (clazz in superClasses.reversed()) {
            fields += clazz.declaredFields
        }

        return fields
    }

    private fun getAllSubtypes(ty: Class<*>) : List<Class<*>> {
        val accum = mutableListOf<Class<*>>()
        getAllSubtypes(ty, accum)
        return accum
    }

    private fun getAllSubtypes(ty: Class<*>, accum: MutableList<Class<*>>) {
        accum += ty

        val directSubclasses = directSubclassMap[ty]
        if (directSubclasses == null)
            return

        for (directSubclass in directSubclasses) {
            getAllSubtypes(directSubclass, accum)
        }
    }
}

class ImportedDefinition(val packet: PacketAnnotation?, val codec: CodecNode) {
    fun toRustDefinition(sb: StringBuilder) {
        if (codec.isTainted())
            return

        (codec as DefinitionCodecNode).toRustDefinition(sb)

        if (packet == null)
            return

        sb.append("impl Packet for ")
        codec.toRustType(sb)
        sb.append(" {\n")
        sb.append("    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {\n")
        sb.append("        name: \"")
        codec.toRustType(sb)
        sb.append("\",\n")
        sb.append("        id: ${packet.id},\n")
        sb.append("        is_compressed: ${packet.compressed},\n")
        sb.append("        max_size: ${packet.maxSize},\n")
        sb.append($"        category: ${packet.categories},\n")
        sb.append("    };\n")
        sb.append("}\n\n")
    }
}

class PacketAnnotation(
    val id: Int,
    val maxSize: Int,
    val compressed: Boolean,
    val categories: String,
)
