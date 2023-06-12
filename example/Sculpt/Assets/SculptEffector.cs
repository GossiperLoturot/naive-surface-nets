using UnityEngine;

[RequireComponent(typeof(Camera))]
public class SculptEffector : MonoBehaviour
{
    public SculptDomain sculptDomain;
    public float radius;
    public float amount;

    void Update()
    {
        var ray = Camera.main.ScreenPointToRay(Input.mousePosition);

        if (Input.GetMouseButton(0) && Physics.Raycast(ray, out var hit))
        {
            sculptDomain.Erase(hit.point, radius, amount * Time.deltaTime);
        }
    }
}
