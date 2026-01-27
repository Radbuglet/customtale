package net.coopfury.customtale_protocol_tools

import java.lang.reflect.Constructor
import java.util.UUID
import kotlin.random.Random

const val DEFAULT_MAX_VAR_LEN: Int = 4096000

enum class OptionSerdeMode {
    Variable,
    Fixed,
}

sealed interface RootCodecNode {
    fun toRustDefinition(sb: StringBuilder)
}

sealed class CodecNode {
    abstract val isDefaultSerializer : Boolean
    abstract val defaultOptionSerdeMode : OptionSerdeMode
    abstract val jvmType : Class<*>

    abstract fun toRustType(sb: StringBuilder)
    abstract fun toRustSerializer(sb: StringBuilder)
    protected abstract fun generateInstance(rng: Random, depth: Int) : Any?
    protected abstract fun isTainted(coinductive: MutableSet<CodecNode>) : Boolean

    fun generateInstance(rng: Random) : Any? {
        return generateInstance(rng, 0)
    }

    fun isTainted() : Boolean {
        return isTainted(mutableSetOf())
    }

    class Struct
        : CodecNode(), RootCodecNode
    {
        private var ctorInner: Constructor<*>? = null
        private var fieldsInner: List<StructField>? = null
        private var defaultOptionSerdeModeInner: OptionSerdeMode? = null

        fun init(ctor: Constructor<*>, fields: List<StructField>, defaultOptionSerdeMode: OptionSerdeMode) {
            ctorInner = ctor
            fieldsInner = fields
            defaultOptionSerdeModeInner = defaultOptionSerdeMode
        }

        val ctor: Constructor<*> get() = ctorInner!!
        val fields: List<StructField> get() = fieldsInner!!

        override val defaultOptionSerdeMode: OptionSerdeMode get() = defaultOptionSerdeModeInner!!
        override val isDefaultSerializer: Boolean get() = true
        override val jvmType: Class<*> get() = ctor.declaringClass

        override fun toRustType(sb: StringBuilder) {
            sb.append("${ctor.declaringClass.simpleName}")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            toRustType(sb)
            sb.append("::codec()")
        }

        override fun toRustDefinition(sb: StringBuilder) {
            sb.append("codec! {\n")
            sb.append("    pub struct ")
            sb.append(ctor.declaringClass.simpleName)
            sb.append("{\n")

            for (field in fields) {
                sb.append("        pub ")
                sb.append(field.name)
                sb.append(": ")
                field.codec.toRustType(sb)

                if (!field.codec.isDefaultSerializer) {
                    sb.append("\n            => ")
                    field.codec.toRustSerializer(sb)
                }
                sb.append(",\n")
            }

            sb.append("    }\n}\n\n")
        }

        override fun generateInstance(rng: Random, depth: Int) : Any? {
            val fields = fields.map { field -> field.codec.generateInstance(rng, depth + 1) }
            return ctor.newInstance(*fields.toTypedArray())
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            if (!coinductive.add(this))
                return false

            for (field in fields) {
                if (field.codec.isTainted(coinductive))
                    return true
            }

            return false
        }
    }

    class StructField(val name: String, val codec: CodecNode)

    class Enum(val type: Class<*>) : CodecNode(), RootCodecNode {
        val variants = type.getField("VALUES").get(null) as Array<*>

        override val isDefaultSerializer: Boolean get() = true
        override val defaultOptionSerdeMode: OptionSerdeMode get() = OptionSerdeMode.Fixed
        override val jvmType: Class<*> get() = type

        override fun toRustType(sb: StringBuilder) {
            sb.append(type.simpleName)
        }

        override fun toRustSerializer(sb: StringBuilder) {
            toRustType(sb)
            sb.append("::codec()")
        }

        override fun toRustDefinition(sb: StringBuilder) {
            sb.append("codec! {\n")
            sb.append("    pub enum ")
            sb.append(type.simpleName)
            sb.append(" {\n")

            for (variant in variants) {
                sb.append("        ")
                sb.append(variant.toString())
                sb.append(",\n")
            }
            sb.append("    }\n}")
        }

        override fun generateInstance(rng: Random, depth: Int): Any? {
            return variants[rng.nextInt(variants.size)]
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return false
        }
    }

    class Optional(val node: CodecNode, val mode: OptionSerdeMode) : CodecNode() {
        override val isDefaultSerializer: Boolean
            get() = node.defaultOptionSerdeMode == mode && node.isDefaultSerializer

        override val defaultOptionSerdeMode: OptionSerdeMode get() = OptionSerdeMode.Variable
        override val jvmType: Class<*> get() = node.jvmType

        constructor(node: CodecNode) : this(node, node.defaultOptionSerdeMode)

        override fun toRustType(sb: StringBuilder) {
            sb.append("Option<")
            node.toRustType(sb)
            sb.append(">")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            node.toRustSerializer(sb)
            sb.append(".nullable_variable()")
        }

        override fun generateInstance(rng: Random, depth: Int) : Any? {
            return if (rng.nextBoolean()) {
                node.generateInstance(rng, depth)
            } else {
                null
            }
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return node.isTainted(coinductive)
        }
    }

    class LeBool : CodecNode() {
        override val isDefaultSerializer: Boolean get() = true
        override val defaultOptionSerdeMode: OptionSerdeMode = OptionSerdeMode.Fixed
        override val jvmType: Class<*> get() = Boolean::class.java

        override fun toRustType(sb: StringBuilder) {
            sb.append("bool")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            sb.append("bool::codec()")
        }

        override fun generateInstance(rng: Random, depth: Int): Any {
            return rng.nextBoolean()
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return false
        }
    }

    class LeByte : CodecNode() {
        override val isDefaultSerializer: Boolean get() = true
        override val defaultOptionSerdeMode: OptionSerdeMode = OptionSerdeMode.Fixed
        override val jvmType: Class<*> get() = Byte::class.java

        override fun toRustType(sb: StringBuilder) {
            sb.append("u8")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            sb.append("u8::codec()")
        }

        override fun generateInstance(rng: Random, depth: Int): Any {
            return rng.nextInt().toByte()
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return false
        }
    }

    class LeShort : CodecNode() {
        override val isDefaultSerializer: Boolean get() = true
        override val defaultOptionSerdeMode: OptionSerdeMode = OptionSerdeMode.Fixed
        override val jvmType: Class<*> get() = Short::class.java

        override fun toRustType(sb: StringBuilder) {
            sb.append("u16")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            sb.append("u16::codec()")
        }

        override fun generateInstance(rng: Random, depth: Int): Any {
            return rng.nextInt().toShort()
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return false
        }
    }

    class LeInt : CodecNode() {
        override val isDefaultSerializer: Boolean get() = true
        override val defaultOptionSerdeMode: OptionSerdeMode = OptionSerdeMode.Fixed
        override val jvmType: Class<*> get() = Int::class.java

        override fun toRustType(sb: StringBuilder) {
            sb.append("u32")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            sb.append("u32::codec()")
        }

        override fun generateInstance(rng: Random, depth: Int): Any {
            return rng.nextInt()
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return false
        }
    }

    class LeLong : CodecNode() {
        override val isDefaultSerializer: Boolean get() = true
        override val defaultOptionSerdeMode: OptionSerdeMode = OptionSerdeMode.Fixed
        override val jvmType: Class<*> get() = Long::class.java

        override fun toRustType(sb: StringBuilder) {
            sb.append("u64")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            sb.append("u64::codec()")
        }

        override fun generateInstance(rng: Random, depth: Int): Any {
            return rng.nextLong()
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return false
        }
    }

    class LeFloat : CodecNode() {
        override val isDefaultSerializer: Boolean get() = true
        override val defaultOptionSerdeMode: OptionSerdeMode = OptionSerdeMode.Fixed
        override val jvmType: Class<*> get() = Float::class.java

        override fun toRustType(sb: StringBuilder) {
            sb.append("f32")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            sb.append("f32::codec()")
        }

        override fun generateInstance(rng: Random, depth: Int): Any {
            return rng.nextFloat()
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return false
        }
    }

    class LeDouble : CodecNode() {
        override val isDefaultSerializer: Boolean get() = true
        override val defaultOptionSerdeMode: OptionSerdeMode = OptionSerdeMode.Fixed
        override val jvmType: Class<*> get() = Double::class.java

        override fun toRustType(sb: StringBuilder) {
            sb.append("f64")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            sb.append("f64::codec()")
        }

        override fun generateInstance(rng: Random, depth: Int): Any {
            return rng.nextDouble()
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return false
        }
    }
    
    class Uuid : CodecNode() {
        override val isDefaultSerializer: Boolean get() = true
        override val defaultOptionSerdeMode: OptionSerdeMode get() = OptionSerdeMode.Fixed
        override val jvmType: Class<*> get() = UUID::class.java

        override fun toRustType(sb: StringBuilder) {
            sb.append("Uuid")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            sb.append("Uuid::codec()")
        }

        override fun generateInstance(rng: Random, depth: Int): Any {
            return UUID.randomUUID()
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return false
        }
    }

    class VarList(val elem: CodecNode, val maxLen: Int) : CodecNode() {
        override val isDefaultSerializer: Boolean get() = maxLen == DEFAULT_MAX_VAR_LEN && elem.isDefaultSerializer
        override val defaultOptionSerdeMode: OptionSerdeMode get() = OptionSerdeMode.Variable
        override val jvmType: Class<*> get() = Array::class.java

        override fun toRustType(sb: StringBuilder) {
            sb.append("Vec<")
            elem.toRustType(sb)
            sb.append(">")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            sb.append("VarArrayCodec::new(")
            elem.toRustSerializer(sb)
            sb.append(", ")
            sb.append(maxLen)
            sb.append(")")
        }

        override fun generateInstance(rng: Random, depth: Int): Any {
            val len = randomLenForDepth(rng, depth)
            val arr = java.lang.reflect.Array.newInstance(elem.jvmType, len)
            (0..<len).forEach { i ->
                java.lang.reflect.Array.set(arr, i, elem.generateInstance(rng, depth + 1))
            }

            return arr
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return elem.isTainted(coinductive)
        }
    }

    class VarMap(val key: CodecNode, val value: CodecNode, val maxLen: Int) : CodecNode() {
        override val isDefaultSerializer: Boolean
            get() = maxLen == DEFAULT_MAX_VAR_LEN && key.isDefaultSerializer && value.isDefaultSerializer

        override val defaultOptionSerdeMode: OptionSerdeMode get() = OptionSerdeMode.Variable
        override val jvmType: Class<*> get() = MutableMap::class.java

        override fun toRustType(sb: StringBuilder) {
            sb.append("HashMap<")
            key.toRustType(sb)
            sb.append(", ")
            value.toRustType(sb)
            sb.append(">")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            sb.append("VarDictionaryCodec::new(")
            key.toRustSerializer(sb)
            sb.append(", ")
            value.toRustSerializer(sb)
            sb.append(", ")
            sb.append(maxLen)
            sb.append(")")
        }

        override fun generateInstance(rng: Random, depth: Int): Any {
            val len = randomLenForDepth(rng, depth)
            val map = mutableMapOf<Any?, Any?>()

            (0..<len).forEach { _ ->
                map[key.generateInstance(rng, depth + 1)] = value.generateInstance(rng, depth + 1)
            }

            return map
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return key.isTainted(coinductive) || value.isTainted(coinductive)
        }
    }

    class VarString(val maxLen: Int) : CodecNode() {
        override val isDefaultSerializer: Boolean get() = maxLen == DEFAULT_MAX_VAR_LEN
        override val defaultOptionSerdeMode: OptionSerdeMode get() = OptionSerdeMode.Variable
        override val jvmType: Class<*> get() = String::class.java

        override fun toRustType(sb: StringBuilder) {
            sb.append("String")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            sb.append("VarStringCodec::new(")
            sb.append(maxLen)
            sb.append(")")
        }

        override fun generateInstance(rng: Random, depth: Int): Any {
            return rng.nextLong().toString()
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return false
        }
    }

    class Tainted : CodecNode() {
        override val isDefaultSerializer: Boolean
            get() = throw NotImplementedError()

        override val defaultOptionSerdeMode: OptionSerdeMode
            get() = throw NotImplementedError()

        override val jvmType: Class<*>
            get() = throw NotImplementedError()

        override fun toRustType(sb: StringBuilder) {
            throw NotImplementedError()
        }

        override fun toRustSerializer(sb: StringBuilder) {
            throw NotImplementedError()
        }

        override fun generateInstance(rng: Random, depth: Int): Any? {
            throw NotImplementedError()
        }

        override fun isTainted(coinductive: MutableSet<CodecNode>): Boolean {
            return true
        }
    }
}

private fun randomLenForDepth(rng: Random, depth: Int) : Int {
    if (depth > 8)
        return 0

    return rng.nextInt(8)
}