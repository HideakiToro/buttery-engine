To enable components to create, delete and reference objects, there has to be some sort of object-registry. This object-registry would handle the creation and deletion of objects. It would also provide each object with an Id, so a Hashmap to store the objects seems reasonable.

The same might be necessary for components themselves, so a component-registry.