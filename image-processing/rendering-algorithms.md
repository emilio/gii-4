---
title: Rendering algorithms
subtitle: Seminar work - Computer Graphics and Image Processing
author:
  - Emilio Cobos Álvarez (<emilio@crisal.io>)
numbersections: true
abstract: |
  With this work I plan to introduce and comment a few rendering algorithms I've
  learnt about during this year.
---

# Real-time rendering algorithms

Real-time rendering of three-dimensional scenes is bottlenecked on hardware in
general, and has some serious time constraints (like having to render a scene
at least 30 times per second).

## Rasterization

Doing a per-pixel approach to real-time rendering is unfeasible, so the usual
rendering method uses **simplified primitives** that the GPU understands
(typically triangles), and it's the GPU the one that takes care of
**rasterising** those primitives into actual pixels.

Current GPUs' pipelines are programmable, so there's a few other steps that
need to run before the primitive assembly and rasterization.

An overview of the rendering pipeline of modern GPUs can be seen in the
[Khronos Group OpenGL wiki][wiki-pipeline-overview].

### Definition

Rasterization is the process of transforming those simple primitives assembled
in the [_Primitive Assembly_ step][assembly] into actual pixels in the
framebuffer.

Actually, in the programmable pipeline, the rasterization process outputs
[_fragments_][fragments] (instead of just pixels), which contain more data than
the pixel color value and position (including user data coming from the vertex
processing stage).

The whole algorithm in modern GPUs is a bit more complex because of the
different configuration (for example, for multisampling OpenGL can produce
multiple [fragments][fragments] on rasterization per triangle per pixel).

### Pseudo-code

The basic rasterization algorithm itself could be simplified (with the above
caveats noted) as something like:

```python
for triangle in triangles_submitted_by_user_in_window_space:
  for pixel in pixels_covered_by(triangle):
    if pixel.z < z_buffer[pixel.x][pixel.y]:
      z_buffer[pixel.x][pixel.y] = pixel.z;
      framebuffer[pixel.x][pixel.y] = run_fragment_shader();
```

## Rendering algorithms for increased realism

As we've seen, normal rasterization for real-time graphics only acts on
primitives, and if we didn't have any external input to the pipeline apart from
the final fragment color, we wouldn't be able to construct realistic real-time
scenes.

The programmable shader model, combined with other features like textures and
similar, allows for more complex algorithms to be implemented to allow more
realism into the scene.

This includes different kinds of techniques like:

 * Shading
 * Texture mapping
 * Bump mapping
 * Shadowing
 * Reflection
 * Fogging
 * Transparency and Translucency
 * Motion blur
 * etc...

I'll explain more of these algorithms if I have the time.

[fragments]: https://www.khronos.org/opengl/wiki/Fragment
[wiki-pipeline-overview]: https://www.khronos.org/opengl/wiki/Rendering_Pipeline_Overview
[assembly]: https://www.khronos.org/opengl/wiki/Primitive_Assembly

# Non-real-time rendering

Non-real-time rendering doesn't have the constraints real-time, and thus can
perform more intensive computation in order to be more realist and accurate.

We'll see some of the most-used algorithms.

## Ray tracing

Ray tracing attempts to simulate the flow of light as particles.

The most basic possible algorithm is trying to cast a light ray per pixel in
an ortographic projection, and then add the contribution of hitting that
surface, then reflect that light and cast the ray from that direction and
position to the next one, and so on, until a maximum iteration bound is reached.

The algorithm in [Rust][rust] looks as follows:

```rust
for (x, y) in final_image {
  let mut ray_position = (x, y, -100)
  let mut ray_direction = (0, 0, 1)
  let mut reflections = 0;
  let mut final_color = (0, 0, 0);
  let mut reflection_factor = 1.0;

  while reflections <= 5 || reflection_factor == 0 {
    let mut current_intersection = None;
    for object in scene {
      match (current_intersection,
             object.intersect_with(ray_position, ray_direction)) {
        // Ray hits on both object, keep the closest one.
        (Some(a), Some(b)) => {
          current_intersection = closest_intersection(ray_position, a, b)
        }
        (None, Some(intersection)) => {
          current_intersection = Some(intersection);
        }
        _ => {}
      }
    }

    let intersection = match intersection {
      Some(i) => i,
      None => break,
    };

    ray_position = intersection;
    ray_direction = reflect(intersected_object, ray_direction);
    final_color += intersected_object.color *
      light_strength * reflection_factor;
    reflection_factor *= intersected_object.reflection_property;
    depth += 1;
  }

  pixel.set(x, y, final_color)
}
```

For what it's worth, I found this [ray tracing
introduction][ray-tracing-introduction] pretty useful for informative purposes.

[ray-tracing-introduction]: https://www.ics.uci.edu/~gopi/CS211B/RayTracing%20tutorial.pdf
[rust]: https://rust-lang.org
