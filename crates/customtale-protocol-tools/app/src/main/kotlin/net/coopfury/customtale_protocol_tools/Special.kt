package net.coopfury.customtale_protocol_tools

import java.lang.reflect.Field

fun escapeNameToIdent(name: String): String {
    return if (name == "Self") "Self_" else "r#${name}"
}

fun isStructSmall(ty: Class<*>) : Boolean {
    return ty.simpleName == "Vector2"
            || ty.simpleName == "Vector2i"
            || ty.simpleName == "Vector2f"
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
            || ty.simpleName == "MouseButtonEvent"
            || ty.simpleName == "WorldInteraction"
            || ty.simpleName == "NearFar"
            || ty.simpleName == "FogOptions"
            || ty.simpleName == "ColorAlpha"
            || ty.simpleName == "Range"
            || ty.simpleName == "Rangef"
            || ty.simpleName == "AmbienceFXSoundEffect"
            || ty.simpleName == "ServerCameraSettings"
}

fun overrideSpecialField(field: Field, importer: Importer) : CodecNode? {
    if (field.declaringClass.name == "com.hypixel.hytale.protocol.TagPattern" && field.name == "not")
        return CodecNode.Optional(CodecNode.Boxed(importer.import(field.declaringClass)))

    if (field.declaringClass.name == "com.hypixel.hytale.protocol.Model" && field.name == "phobiaModel")
        return CodecNode.Optional(CodecNode.Boxed(importer.import(field.declaringClass)))

    if (field.declaringClass.name == "com.hypixel.hytale.protocol.ForkedChainId" && field.name == "forkedId")
        return CodecNode.Optional(CodecNode.Boxed(importer.import(field.declaringClass)))

    if (field.declaringClass.name == "com.hypixel.hytale.protocol.Asset") {
        if (field.name == "hash")
            return CodecNode.FixedString(64)

        if (field.name == "name")
            return CodecNode.VarString(512)
    }

    return null
}
