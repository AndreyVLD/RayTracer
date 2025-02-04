# Ray Tracer

![final_scene.png](outputs/final_scene.png)
A ray tracer implemented in Rust, inspired by the *Ray Tracing in One Weekend* book.
The project renders 3D scenes with spheres, boxes, materials, basic shading and lighting.

## Features

- Ray-sphere and ray-quadrilateral intersection
- Configurable camera with perspective projection
- Reflections and Refractions
- Light sources
- Lambertian model of shading
- Multi Sampled Anti Aliasing
- Multiple surface materials such as: diffuse, metallic, dielectric and isotropic.
- Volumetric rendering and fog
- Depth of field
- Texture mapping
- Instanceable objects: rotate and translate
- Gradient Background
- Image output to PNG

## Getting Started

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

### Build and Run

1. Clone the repository:

```shell
git clone https://github.com/AndreyVLD/RayTracer
cd RayTracer
```

2. Build and run the project:

```shell
cargo run
```

3. View the output image:

- The rendered scene is saved as `output.png` in the project directory.

## File Structure

- `src\`
    - `main.rs`: Entry point
    - `camera.rs`: Camera setup and ray generation
    - `hit.rs`: Struct for ray hits information and trait of hittable objects
    - `materials.rs`: Structs of surfaces used by hittable objects
    - `ray.rs`: Ray struct and its implementation functions
    - `scene.rs`: Scene setup and rendering
    - `texture.rs`: Texture struct and its implementation functions
    - `transformation.rs`: Structs for rotation and translation of objects and their implementation functions
    - `utils.rs`: Utility maths functions.
    - `vector3.rs`: 3D Vector struct and its implementation functions
    - `shapes\`
        - `box_quad.rs`: A struct for box formed from 6 quadrilaterals and its implementation functions
        - `volume.rs`: Struct for Constant Medium rendering and its implementation functions
        - `quad.rs`: Struct for a 4 vertex quadrilateral
        - `sphere.rs`: Sphere struct and its implementation functions

## Gallery

![spheres.png](outputs/spheres.png)
![basic_lights_improved.png](outputs/basic_lights_improved.png)
![colored_lights.png](outputs/colored_lights.png)
![cornell_box_improved.png](outputs/cornell_box_improved.png)
![cornell_fog_enhanced.png](outputs/cornell_fog_enhanced.png)