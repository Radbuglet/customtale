package net.coopfury.customtale_protocol_tools

import java.lang.reflect.Constructor
import java.util.UUID
import kotlin.random.Random

enum class OptionSerdeMode {
    Variable,
    Fixed,
}

sealed class CodecNode {
    abstract val isDefaultSerializer : Boolean
    abstract val defaultOptionSerdeMode : OptionSerdeMode
    abstract val jvmType : Class<*>

    abstract fun toRustType(sb: StringBuilder)
    abstract fun toRustSerializer(sb: StringBuilder)
    abstract fun generateInstance(rng: Random, depth: Int) : Any?

    class Struct(val ctor: Constructor<*>, override val defaultOptionSerdeMode: OptionSerdeMode) : CodecNode() {
        private var fieldsInner: List<CodecNode>? = null

        val fields: List<CodecNode> get() = fieldsInner!!

        fun initFields(fields: List<CodecNode>) {
            fieldsInner = fields
        }

        override val isDefaultSerializer: Boolean get() = true
        override val jvmType: Class<*> get() = ctor.declaringClass

        override fun toRustType(sb: StringBuilder) {
            sb.append("${ctor.declaringClass.simpleName}")
        }

        override fun toRustSerializer(sb: StringBuilder) {
            toRustType(sb)
            sb.append("::codec()")
        }

        override fun generateInstance(rng: Random, depth: Int) : Any? {
            val fields = fields.map { i -> i.generateInstance(rng, depth + 1) }
            return ctor.newInstance(*fields.toTypedArray())
        }
    }

    class Optional(val node: CodecNode, val mode: OptionSerdeMode) : CodecNode() {
        override val isDefaultSerializer: Boolean
            get() = node.defaultOptionSerdeMode == mode && node.isDefaultSerializer

        override val defaultOptionSerdeMode: OptionSerdeMode get() = OptionSerdeMode.Variable
        override val jvmType: Class<*> get() = node.jvmType

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
    }

    class VarList(val elem: CodecNode, val maxLen: Int) : CodecNode() {
        override val isDefaultSerializer: Boolean get() = maxLen == 4096000 && elem.isDefaultSerializer
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
            if (depth > 8) {
                return emptyList<Any?>()
            }

            val len = rng.nextInt(8)
            val arr = java.lang.reflect.Array.newInstance(elem.jvmType, len)
            (0..<len).forEach { i ->
                java.lang.reflect.Array.set(arr, i, elem.generateInstance(rng, depth + 1))
            }

            return arr
        }
    }

    class VarMap(val key: CodecNode, val value: CodecNode, val maxLen: Int) : CodecNode() {
        override val isDefaultSerializer: Boolean
            get() = maxLen == 4096000 && key.isDefaultSerializer && value.isDefaultSerializer

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

        override fun generateInstance(rng: Random, depth: Int): Any? {
            TODO("Not yet implemented")
        }
    }
}
