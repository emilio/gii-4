# Computer graphics: Second meeting

(For some reason, I think the All-Hands, I wasn't in the first one)

## Textures

 * Adding detail to geometric objects without increasing geometric complexity.
 * Texture mapping: map from property of fragment to texel.

### Two phase texture mapping
 * S-mapping: Mapping a 2D texture onto an auxiliary 3D surface that surrounds
   the target object.

   * Usually cube, cylinder, or sphere.

 * O-mapping: Mapping from that 3D surface onto the target object.

   * Pretty similar to environment mapping, but without needing a first
     all-the-scene render pass.

   * You can map the surface however you want. Some examples:

     * Mapping in mirror direction.
     * Mapping in normal direction on object.
     * Mapping from object center
     * Mapping in normal direction on auxiliary surface

### Environment mapping.

(I've written already about this).

### Bump and normal mapping

Mapping perturbed normals of a surface _without modifying the fragment
position_.

Bump mapping: 1D texture that contains height of the bump map (you calculate the
normal in the fragment shader).

Normal mapping: 3D texture that contains the normal directly.

# Curves

Exercise: Calculate $p(0.3)$ on a cubic Hermite curve if the control points and
tangents are:

 * $p = [ (0, 1, 0), (-2, 3, 2)$
 * $p' = [ (-1, 1, 0), (1, 0, 2) ]$

