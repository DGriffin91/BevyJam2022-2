import os
import subprocess
import sys
import fileinput

path = sys.argv[1] #".././assets/models/FlightHelmet"

dir_list = os.listdir(path)

for item in dir_list:
    parts = item.split(".")
    if len(parts) < 2:
        continue
    file, ext = parts

    if ext.lower() == "gltf" and "_ktx2" not in file.lower():
        with open(f"{path}/{item}") as f:
            gltf = f.read()
        with open(f"{path}/{file}_ktx2.{ext}", 'w') as f:
            f.write(gltf.replace(".png", ".ktx2"))
        continue
    
    if ext.lower() != "png":
        continue
    if ext.lower() == "jpg":
        print("jpg images not supported!")
        continue
    
    #print(item)
    if "_normal" in file.lower():
        fmt = "bc5"
    else:
        fmt = "bc7"

    cmd = [
        "kram", 
        "encode",
        "-type",
        "2d",
        "-f",
        fmt,
        "-mipmin",
        "2",
        "-zstd",
        "0",
        "-i",
        f"{path}/{item}",
        "-o",
        f"{path}/{file}.ktx2",
        ]
    print(cmd)
    subprocess.run(cmd)

    