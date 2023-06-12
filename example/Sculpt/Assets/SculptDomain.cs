using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;
using Unity.Collections;
using Unity.Collections.LowLevel.Unsafe;

[RequireComponent(typeof(MeshFilter), typeof(MeshCollider))]
public class SculptDomain : MonoBehaviour
{
    float[] voxelArray;
    int size;

    Mesh mesh;
    MeshFilter mf;
    MeshCollider mc;

    [DllImport("NaiveSurfaceNets", EntryPoint="naive_surface_nets_raw")]
    static unsafe extern void NaiveSurfaceNets(float* voxels, int size, Vector3* vertices, int* vertexCount, ushort* triangles, int* triangleCount);

    void Start()
    {
        voxelArray = new float[4096];
        size = 16;

        for (var x = 0; x < size; x++)
        for (var y = 0; y < size; y++)
        for (var z = 0; z < size; z++)
        {
            var xh = x - 0.5f * size;
            var yh = y - 0.5f * size;
            var zh = z - 0.5f * size;
            var r = 0.5f * size - 1f;
            voxelArray[x + y * size + z * size * size] = xh * xh + yh * yh + zh * zh - r * r;
        }

        mesh = new Mesh();
        mesh.MarkDynamic();
        mf = GetComponent<MeshFilter>();
        mc = GetComponent<MeshCollider>();

        Remesh();
    }

    public void Erase(Vector3 point, float radius, float amount)
    {
        var x = (int)(point.x + 0.5f);
        var y = (int)(point.y + 0.5f);
        var z = (int)(point.z + 0.5f);

        for (var xp = Mathf.Max(0, Mathf.FloorToInt(x - radius)); xp < Mathf.Min(Mathf.CeilToInt(x + radius), size); xp++)
        for (var yp = Mathf.Max(0, Mathf.FloorToInt(y - radius)); yp < Mathf.Min(Mathf.CeilToInt(y + radius), size); yp++)
        for (var zp = Mathf.Max(0, Mathf.FloorToInt(z - radius)); zp < Mathf.Min(Mathf.CeilToInt(z + radius), size); zp++)
        {
            var xd = xp - (point.x + 0.5f);
            var yd = yp - (point.y + 0.5f);
            var zd = zp - (point.z + 0.5f);
            var normalized = Mathf.Max(0f, 1f - (xd * xd + yd * yd + zd * zd) / (radius * radius));
            voxelArray[xp + yp * size + zp * size * size] += normalized * amount;
        }

        Remesh();
    }

    unsafe void Remesh()
    {
        var vertexArray = new NativeArray<Vector3>(4096, Allocator.Temp, NativeArrayOptions.ClearMemory);
        var triangleArray = new NativeArray<ushort>(4096 * 18, Allocator.Temp, NativeArrayOptions.ClearMemory);

        var vertices = (Vector3*)NativeArrayUnsafeUtility.GetUnsafePtr(vertexArray);
        var triangles = (ushort*)NativeArrayUnsafeUtility.GetUnsafePtr(triangleArray);

        int vertexCount;
        int triangleCount;

        fixed (float* voxels = voxelArray) NaiveSurfaceNets(voxels, size, vertices, &vertexCount, triangles, &triangleCount);

        mesh.SetVertices(vertexArray);
        mesh.SetIndices(triangleArray, MeshTopology.Triangles, 0);
        mesh.RecalculateNormals();
        mesh.RecalculateTangents();

        mf.sharedMesh = mesh;
        mc.sharedMesh = mesh;

        vertexArray.Dispose();
        triangleArray.Dispose();
    }
}
