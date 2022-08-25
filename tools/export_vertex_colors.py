#path_to_blender_python_bin/python.exe -m pip install zstd

import bpy
import numpy as np
import zstd

active_obj = bpy.context.active_object

data = bytearray()
for d in active_obj.data.attributes['Bake'].data:
    nparray = np.array([d.color[0],d.color[1],d.color[2]], dtype=np.float32)
    #print(nparray)
    data.extend(nparray.tobytes())

data = bytes(data)

f = open("bytes", "wb")
f.write(zstd.compress(data, 1))
f.close()