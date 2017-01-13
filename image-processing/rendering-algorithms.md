---
title: Rendering algorithms
subtitle: Seminar work - Computer Graphics and Image Processing
author:
  - Emilio Cobos Álvarez (<emilio@crisal.io>)
numbersections: true
colorlinks: true
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

### Shadow mapping

Shadow-mapping is the go-to method to implement semi-realistic shadows in
real-time computer graphics.

The idea is to project with an orthographic projection the scene from the
light's point of view into the depth buffer, and then save that depth buffer
into a float texture, that we'll call the _shadow map_.

That way, you get a representation of the scene's depth.

Then in the second pass (the one that properly renders to the final draw
buffer), you look at that shadow map and compare the fragment's actual depth
(as-if it was rendered with orthographic projection), and the depth saved in the
depth map.

If there's a smaller depth value in the map, that means that there's an object
before the current fragment from the light's position, and thus you can reduce
the diffuse and reflective components of the light to get a "shadow-like"
effect.

#### Percentage-closer filtering (PCF)

Shadow mapping have a relatively serious problem, that is that the shadows by
default seem really unrealistic.

One of the ways to improve this is the technique called percentage-closer
filtering (PFC).

It's described in depth in [this GPUGems article][pcf-gpugems], but the basic
idea is sampling the nearby pixels in the shadow map in order to give a "fading"
effect to the shadow.

I have a [working implementation][pcf-impl] of both shadow mapping and PCF in my
Computer Graphics repo.

That implementation I chose for PCF is pretty simple and just samples the eight
surrounding pixels of the current fragment, but the look is much better than
without the technique.

[pcf-gpugems]: http://http.developer.nvidia.com/GPUGems/gpugems_ch11.html
[pcf-impl]: https://github.com/emilio/cg-2016/blob/master/res/fragment.glsl#L12

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

I was wondering while reading about ray tracing, so I wrote a [very simple ray
tracer](https://github.com/emilio/toy-ray-tracer) in [Rust][rust].

You can see the main algorithm [directly in the
source](https://github.com/emilio/toy-ray-tracer/blob/master/tracer/src/lib.rs#L266).

Of course, on top of that you need to implement a reflection model and what not
(I did a very basic Phong shading), but that's the basic idea.

For what it's worth, I found this [ray tracing
introduction][ray-tracing-introduction] pretty useful for informative purposes.

## Ray casting

Ray casting consists mainly in ray tracing, but only emitting primary rays. In
the previous code snippet, we'd change `MAX_REFLECTIONS` to be `0`.

This means that there's no reflection going on.

## Scanline rendering

[Scanline rendering][sr-wiki] is another relatively common rendering algorithm,
that doesn't go neither polygon-by-polygon, neither pixel-by-pixel.

It goes instead row-by-row (line by line if you wish), calculating all the edges
of the polygons in the scene that intersect with that line, sorted by
x coordinate.

Once you have that list of polygons, you go left-to-right painting the pixels
between intersections.

You may need depth information if more than two polygons intersect in a line,
but it's easy to use a depth buffer for that.

I found [this Bielefeld University lecture][sr-bielefeld] quite nice for
a visual explanation, and also some optimizations that may be done, in the
intersection computation phase.

Also, it's quite easy to combine with shading techniques like the Phong
reflection model and what not.

[sr-wiki]: https://en.wikipedia.org/wiki/Scanline_rendering
[sr-bielefeld]: https://www.techfak.uni-bielefeld.de/ags/wbski/lehre/digiSA/WS0607/3DVRCG/Vorlesung/13.RT3DCGVR-vertex-2-fragment.pdf

## Environment mapping

Environment mapping is an alternative to ray tracing in order to provide
reflection suitable for real-time rendering.

The intended use of environment mapping is to simulate reflections or lighting
upon objects without going through expensive ray-tracing or lighting
calculations[^env-mapping-description].

The basic idea is to render the environment from the point of view of the model
in different ways, into a texture, and then sampling that texture when rendering
the object to get a reflection effect easily.

### Cube mapping

Cube mapping consists in rendering the scene from the point of view of the
object in the six directions, "snapshotting" the scene in one side of a cube
each.

This is the technique used to implement the sky box in our demo application for
Computer Graphics (just in a static rather than dynamic way).

Once you have the six points of view mapped into a texture, you sample that
texture in order to get the reflected scene.

There's a [comprehensive example from the University of
Illinois][illinois-env-mapping].

Obviously, environment mapping is a bit trickier than just rendering a skybox
(because when rendering a skybox you only need to render a cube, and you don't
need to care about projecting it itself).

Using OpenGL's `samplerCube` is relatively easy to calculate the proper
reflection for a model, you just need to reflect the view ray on the model, then
sample the texture at that position:

```glsl
vec3 I = normalize(fPosition - cameraPos);
vec3 R = reflect(I, normalize(fNormal));
oFragColor = texture(uCubeMap, R);
```

[learnopengl.com][log-env-mapping] gives a pretty good introduction to cube
mapping and environment mapping, and also provides good insight in the
disadvantages of dynamic environment mapping:

> Using framebuffers it is possible to create a texture of the scene for all
> 6 different angles from the object in question and store those in a cubemap
> each render iteration. We can then use this (dynamically generated) cubemap to
> create realistic reflection and refractive surfaces that include all other
> objects. This is called dynamic environment mapping, because we dynamically
> create a cubemap of an object's surroundings and use that as its environment
> map.
>
> While it looks great, it has one enormous disadvantage: we have to render the
> scene 6 times per object using an environment map, which is an enormous
> performance penalty on your application. Modern applications try to use the
> skybox as much as possible and where possible pre-compile cubemaps wherever
> they can to still sort-of create dynamic environment maps. While dynamic
> environment mapping is a great technique, it requires a lot of clever tricks
> and hacks to get it working in an actual rendering application without too
> many performance drops.

[illinois-env-mapping]: https://courses.engr.illinois.edu/cs418/slides/Lecture%2027%20-%20Environment%20Mapping%20in%20WebGL.pdf
[log-env-mapping]: https://learnopengl.com/#!Advanced-OpenGL/Cubemaps

### Spherical mapping

One of the possible ways to implement environment mapping is using a spherical
reflection map.

This technique is called *spherical environment mapping*, and it's described
quite well [here][sph-env]. That's a little more complex than what it needs to
be (a relatively old description can be found [here][wiki-sph-env]).

Conceptually it can be seen as reflecting the cube map into a sphere, then
flattening the sphere into 2D, then sample from there.

[^env-mapping-description]: http://www.reindelsoftware.com/Documents/Mapping/Mapping.html

[ray-tracing-introduction]: https://www.ics.uci.edu/~gopi/CS211B/RayTracing%20tutorial.pdf
[sph-env]: https://www.clicktorelease.com/blog/creating-spherical-environment-mapping-shader
[wiki-sph-env]: http://www.reindelsoftware.com/Documents/Mapping/Mapping.html
[rust]: https://rust-lang.org

# Common rendering data structures

This space is something I've wanted to investigate for a while, because I've
seen some of them [used to speed up a renderer I've contributed to][webrender].

[webrender]: https://github.com/servo/webrender

## Quad tree

A quad tree is a data structure used to divide a 2d space into quads. I've seen
it used to implement a level-of-detail terrain, where I divided the terrain in
four quads, then the closer the camera was, the more I divided the terrain. Each
leaf in the tree had a fixed number of triangles.

That way, you end up with a tree that looks like this (the `o` is the camera):

```
+-----+-----+------------+-----------------------+
|    o|     |            |                       |
|     |     |            |                       |
+-----+-----+            |                       |
|     |     |            |                       |
|     |     |            |                       |
+-----+-----+------------+                       |
|           |            |                       |
|           |            |                       |
|           |            |                       |
|           |            |                       |
|           |            |                       |
+-----------+------------+-----------------------+
|                        |                       |
|                        |                       |
|                        |                       |
|                        |                       |
|                        |                       |
|                        |                       |
|                        |                       |
|                        |                       |
|                        |                       |
|                        |                       |
|                        |                       |
+------------------------+-----------------------+
```

Partitioning 2d space this way, you get a nice level of detail effect, without
having to render all the triangles in the terrain.

## AABB tree

An AABB (Axis-Aligned Bounding Box) tree, is a data structure used (among other
things) to optimize the detection of intersections in a 3d scene.

Conceptually, it's a nested tree of boxes, so you can stop traversing most of
them when you test the outer box.

Checking whether a line intersects an axis aligned bounding box is easy, so
computations can be done relatively easy.

The quad tree can be seen as a 2d AABB tree, with a fixed number of child nodes.

The invariant is, just as in the quad tree, that the bounding boxes of the
children of a given node are contained in the bounding box of the parent node.

There's a [nice visualization][cgal-aabb] and a [reference
implementation][cgal-aabb-src] in the CGAL library.

[cgal-aabb]: http://doc.cgal.org/latest/AABB_tree/index.html
[cgal-aabb-src]: https://github.com/CGAL/cgal/tree/master/AABB_tree
