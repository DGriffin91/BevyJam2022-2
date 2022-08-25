#path_to_blender_python_bin/python.exe -m pip install zstd

import bpy
import numpy as np
import os
import zstd

filepath = bpy.data.filepath
directory = os.path.dirname(filepath)

active_obj = bpy.context.active_object

data = active_obj.data.attributes['Bake'].data

with open(os.path.join(directory, "bytes"), "rb") as f:
    fdata = zstd.decompress(f.read())
    nparray = np.frombuffer(fdata, dtype=np.float32)
    c = 0
    for i in range(0,len(nparray),3):
        data[c].color[0] = nparray[i+0]
        data[c].color[1] = nparray[i+1]
        data[c].color[2] = nparray[i+2]
        data[c].color[3] = 1.0
        c += 1
