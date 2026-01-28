package net.coopfury.customtale_protocol_tools

import java.lang.reflect.Field

fun escapeNameToIdent(name: String): String {
    return if (name == "Self") "Self_" else "r#${name}"
}

fun isStructSmall(ty: Class<*>) : Boolean {
    return ty.simpleName == "Vector2"
            || ty.simpleName == "Vector2i"
            || ty.simpleName == "Vector3d"
            || ty.simpleName == "Vector3f"
            || ty.simpleName == "Vector3i"
            || ty.simpleName == "Position"
            || ty.simpleName == "Direction"
            || ty.simpleName == "Transform"
            || ty.simpleName == "InstantData"
            || ty.simpleName == "BlockPosition"
            || ty.simpleName == "ColorLight"
            || ty.simpleName == "SavedMovementStates"
            || ty.simpleName == "VelocityConfig"
            || ty.simpleName == "Hitbox"
            || ty.simpleName == "Color"
            || ty.simpleName == "Tint"
            || ty.simpleName == "BlockFlags"
            || ty.simpleName == "ModelTransform"
            || ty.simpleName == "SleepClock"
            || ty.simpleName == "EasingConfig"
            || ty.simpleName == "MovementStates"
            || ty.simpleName == "HalfFloatPosition"
            || ty.simpleName == "TeleportAck"
            || ty.simpleName == "BlockRotation"
}

fun overrideSpecialField(field: Field, importer: Importer) : CodecNode? {
    if (field.declaringClass.simpleName == "TagPattern" && field.name == "not")
        return CodecNode.Optional(CodecNode.Boxed(importer.import("com.hypixel.hytale.protocol.TagPattern")))

    if (field.declaringClass.simpleName == "Model" && field.name == "phobiaModel")
        return CodecNode.Optional(CodecNode.Boxed(importer.import("com.hypixel.hytale.protocol.Model")))

    if (field.declaringClass.simpleName == "ForkedChainId" && field.name == "forkedId")
        return CodecNode.Optional(CodecNode.Boxed(importer.import("com.hypixel.hytale.protocol.ForkedChainId")))

    return null
}
