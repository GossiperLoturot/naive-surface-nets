using UnityEngine;

public class OrbitTransform : MonoBehaviour
{
    public Transform origin;
    public float range;
    public float sensitivity;

    private Vector3 eulerAngles;

    void Update()
    {
        if (Input.GetMouseButton(2))
        {
            eulerAngles += new Vector3(-Input.GetAxis("Mouse Y"), Input.GetAxis("Mouse X"), 0) * Time.deltaTime * 360 * sensitivity;
            transform.position = origin.position + Quaternion.Euler(eulerAngles) * Vector3.back * range;
            transform.rotation = Quaternion.LookRotation(origin.position - transform.position);
        } 
    }
}
