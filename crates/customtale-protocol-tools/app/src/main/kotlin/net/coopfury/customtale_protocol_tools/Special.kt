package net.coopfury.customtale_protocol_tools

import java.lang.reflect.Field

const val PACKET_PKG_ROOT: String = "com.hypixel.hytale.protocol"

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
            || ty.simpleName == "Rangeb"
            || ty.simpleName == "FloatRange"
            || ty.simpleName == "Rangef"
            || ty.simpleName == "AmbienceFXSoundEffect"
            || ty.simpleName == "ServerCameraSettings"
            || ty.simpleName == "AssetEditorRebuildCaches"
            || ty.simpleName == "AssetEditorPreviewCameraSettings"
            || ty.simpleName == "BlockMovementSettings"
            || ty.simpleName == "Edge"
            || ty.simpleName == "IntersectionHighlight"
            || ty.simpleName == "ClampConfig"
            || ty.simpleName == "SoundEventLayerRandomSettings"
            || ty.simpleName == "InitialVelocity"
            || ty.simpleName == "RangeVector2f"
            || ty.simpleName == "RangeVector3f"
            || ty.simpleName == "ParticleCollision"
            || ty.simpleName == "ParticleAnimationFrame"
            || ty.simpleName == "PortalState"
            || ty.simpleName == "PhysicsConfig"
            || ty.simpleName == "MovementSettings"
            || ty.simpleName == "BlockMount"
            || ty.simpleName == "EditorSelection"
            || ty.simpleName == "FluidFXMovementSettings"
            || ty.simpleName == "BlockPlacementSettings"
            || ty.simpleName == "WiggleWeights"
            || ty.simpleName == "Size"
}

fun overrideSpecialField(field: Field, importer: Importer) : CodecNode? {
    if (field.declaringClass.name == "$PACKET_PKG_ROOT.TagPattern" && field.name == "not")
        return CodecNode.Optional(CodecNode.Boxed(importer.import(field.declaringClass)))

    if (field.declaringClass.name == "$PACKET_PKG_ROOT.Model" && field.name == "phobiaModel")
        return CodecNode.Optional(CodecNode.Boxed(importer.import(field.declaringClass)))

    if (field.declaringClass.name == "$PACKET_PKG_ROOT.ForkedChainId" && field.name == "forkedId")
        return CodecNode.Optional(CodecNode.Boxed(importer.import(field.declaringClass)))

    if (field.declaringClass.name == "$PACKET_PKG_ROOT.Asset") {
        if (field.name == "hash")
            return CodecNode.FixedString(64)

        if (field.name == "name")
            return CodecNode.VarString(512)
    }

    if (field.declaringClass.name == "$PACKET_PKG_ROOT.HostAddress") {
        if (field.name == "host")
            return CodecNode.VarString(256)
    }

    if (field.declaringClass.name == "$PACKET_PKG_ROOT.packets.connection.Connect") {
        if (field.name == "clientVersion")
            return CodecNode.FixedString(20)

        if (field.name == "username")
            return CodecNode.VarString(16)

        if (field.name == "identityToken")
            return CodecNode.Optional(CodecNode.VarString(8192))

        if (field.name == "language")
            return CodecNode.VarString(16)

        if (field.name == "referralData")
            return CodecNode.Optional(CodecNode.VarByteArray(4096))
    }

    return null
}
