use crate::codec;

codec! {
    #[derive(Copy)]
    pub struct Range {
        @small = true;
        pub min: i32,
        pub max: i32,
    }

    #[derive(Copy)]
    pub struct Rangef {
        @small = true;
        pub min: f32,
        pub max: f32,
    }

    #[derive(Copy)]
    pub struct Rangeb {
        @small = true;
        pub min: u8,
        pub max: u8,
    }

    #[derive(Copy)]
    pub struct FloatRange {
        @small = true;
        pub inclusive_min: f32,
        pub inclusive_max: f32,
    }

    #[derive(Copy)]
    pub struct RangeVector2f {
        @small = true;
        pub x: Rangef,
        pub y: Rangef,
    }

    #[derive(Copy)]
    pub struct Color {
        @small = true;
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    #[derive(Copy)]
    pub struct Vector2f {
        @small = true;
        pub x: f32,
        pub y: f32,
    }

    #[derive(Copy)]
    pub struct Vector3f {
        @small = true;
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Copy)]
    pub struct Vector3i {
        @small = true;
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    #[derive(Copy)]
    pub struct Direction {
        @small = true;
        pub yaw: f32,
        pub pitch: f32,
        pub roll: f32,
    }

    #[derive(Copy)]
    pub struct ColorLight {
        @small = true;
        pub radius: u8,
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    pub struct NearFar {
        @small = true;
        pub near: f32,
        pub far: f32,
    }
}
