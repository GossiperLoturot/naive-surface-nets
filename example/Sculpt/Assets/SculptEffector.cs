using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.InputSystem;

[RequireComponent(typeof(Camera))]
public class SculptEffector : MonoBehaviour
{
    public SculptDomain sculptDomain;
    public float radius = 1f;
    public float amount = 1f;

    Vector2 point;
    bool pressed;

    Camera camera;

    void Start()
    {
        camera = GetComponent<Camera>();
    }

    public void PointCallback(InputAction.CallbackContext context)
    {
        point = context.ReadValue<Vector2>();
    }

    public void ClickCallback(InputAction.CallbackContext context)
    {
        pressed = 0.5f < context.ReadValue<float>();
    }

    void Update()
    {
        if (!pressed) return;

        var ray = camera.ScreenPointToRay(point);

        if (Physics.Raycast(ray, out var hit)) sculptDomain.Erase(hit.point, radius, amount * Time.deltaTime);
    }
}
