using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.InputSystem;

public class OrbitTransform : MonoBehaviour
{
    public Transform origin;
    public Vector3 offset;
    public float sensitivity;

    Vector2 look;
    bool pressed;

    Vector3 eulerAngles;

    public void LookCallback(InputAction.CallbackContext context)
    {
        look = context.ReadValue<Vector2>();
    }

    public void MiddleClickCallback(InputAction.CallbackContext context)
    {
        pressed = 0.5f < context.ReadValue<float>();
    }

    void Update()
    {
        if (pressed)
        {
            eulerAngles += new Vector3(-look.y, look.x, 0f) * Time.deltaTime * 360f * sensitivity;

            transform.position = origin.position + Quaternion.Euler(eulerAngles) * offset;
            transform.rotation = Quaternion.LookRotation(origin.position - transform.position);
        } 
    }
}
