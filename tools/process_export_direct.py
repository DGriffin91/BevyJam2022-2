PREVIEW_MODE = True
SPLIT_EVERYTHING = True
SPLIT_ANYTHING = True
DENOISE_TEX = True
DENOISE_VERT = False
TEX_SAMPLES = 512
VERT_SAMPLES = 4096
MEDIAN_FILTER = False
RES_MULT = 2.0
VERT_COL_METHOD = "POINT" #'POINT', 'EDGE', 'FACE', 'CORNER', 'CURVE', 'INSTANCE'
VERT_COL_CONVERT_TO = "CORNER"

# Not using an operator because it seems that apply modifiers is broken if used in a operator

import bpy
import os
import math

C = bpy.context
view_layer = bpy.context.view_layer
scene = bpy.context.scene

def deselect():
    bpy.ops.object.select_all(action='DESELECT')

def apply_modifiers_scale_deselect(ob):
    deselect()
    ob.select_set(True)
    view_layer.objects.active = ob
    if ob.data.users > 1:
        ob.data = ob.data.copy() #make_single_user
    for m in ob.modifiers:
        if not m.show_in_editmode or not m.show_render:
            continue
        print(m.name)
        bpy.ops.object.modifier_apply(modifier=m.name, single_user=True)
    bpy.ops.object.transform_apply(location=False, rotation=False, scale=True)
    deselect()

def create_material_if_needed(ob):
    if ob.data.materials is None:
        mat = bpy.data.materials.new(name="Material")
        # no slots
        ob.data.materials.append(mat)
        mat.use_nodes = True
    elif len(ob.data.materials) < 1:
        mat = bpy.data.materials.new(name="Material")
        ob.data.materials.append(mat)
        # assign to 1st material slot
        ob.data.materials[0] = mat
        mat.use_nodes = True
    elif ob.data.materials[0] is None:
        mat = bpy.data.materials.new(name="Material")
        ob.data.materials[0] = mat
        ob.data.materials[0].use_nodes = True
    else:
        ob.data.materials[0].use_nodes = True

    if len(ob.data.materials) < 1 or ob.data.materials is None:
        raise Exception("create_material_if_needed didn't work")

#https://docs.blender.org/api/current/bpy.types.AttributeGroup.html
def setup_vertex_bake(ob):
    mesh = ob.data
    if "Bake" not in mesh.attributes:
        mesh.attributes.new("Bake", "BYTE_COLOR", VERT_COL_METHOD) #CORNER
    
    bake_index = mesh.attributes.keys().index("Bake")
    mesh.attributes.render_color_index = bake_index

def setup_edge_split(ob):
    if ob.data.use_auto_smooth or SPLIT_EVERYTHING:
        mod = ob.modifiers.new("EdgeSplit", 'EDGE_SPLIT')
        mod.split_angle = ob.data.auto_smooth_angle

def smart_project(angle_limit=66, island_margin=0.04):
    bpy.ops.object.mode_set(mode='EDIT')
    bpy.ops.mesh.select_all(action='SELECT')
    bpy.ops.uv.smart_project(angle_limit=math.radians(angle_limit), 
                             island_margin=island_margin, 
                             correct_aspect=True)
    bpy.ops.object.mode_set(mode='OBJECT')

def smart_project_deselect(active, others, vlayer, angle_limit=66, island_margin=0.04):
    deselect()
    for ob in others:
        ob.select_set(True)
    active.select_set(True)
    vlayer.objects.active = active
    smart_project(angle_limit=angle_limit, island_margin=island_margin)
    deselect()

def create_tex_bake_node(ob, name, img):
    create_material_if_needed(ob)

    # The object may have multiple materials, put the texture to render to on all of them
    for i, mat in enumerate(ob.data.materials):
        if mat is None:
            continue
        mat = ob.data.materials[i] = mat.copy()
            
        mat.use_nodes = True
        nodes = mat.node_tree.nodes
        texture_node = nodes.get(name)
        if texture_node is None:
            texture_node = nodes.new('ShaderNodeTexImage')
            texture_node.name = name
        texture_node.select = True
        nodes.active = texture_node
        texture_node.image = img
        texture_node.image.colorspace_settings.name = 'sRGB'

def save_image_use_saved(img, path):
    img.save_render(path)
    img.source = "FILE"
    img.filepath = path
    img.reload()

def bake_proc_median(img):
    print(os.system(f"bake_proc.exe \"{str(img.filepath)}\" \"{str(img.filepath)}\""))
    img.reload()

def get_pass_filter(bake_pass):
    # COLOR, ROUGHNESS, NORMAL, EMIT, COMBINED
    # for metallic, you'll probably have 
    # to connect the metallic inputs to an emit
    #https://docs.blender.org/api/current/bpy.ops.object.html#bpy.ops.object.bake
    pass_filter = set()
    if bake_pass == "ROUGHNESS":
        bake_type = "ROUGHNESS"
    elif bake_pass == "COLOR":
        pass_filter = {"COLOR"}
        bake_type = "DIFFUSE"
    elif bake_pass == "NORMAL":
        bake_type = "NORMAL"
    elif bake_pass == "EMIT":
        pass_filter = {"COLOR"}
        bake_type = "EMIT"
    elif bake_pass == "COMBINED":
        pass_filter = {"DIRECT", "INDIRECT", 
        "DIFFUSE", "GLOSSY", "TRANSMISSION", "EMIT"}
        bake_type = "COMBINED"
    return (bake_type, pass_filter)

def bake(bake_pass, target):
    pass_filter = get_pass_filter(bake_pass)
    bpy.ops.object.bake(type=pass_filter[0], pass_filter=pass_filter[1], target=target)

def bake_pass_tex(ob, basedir, name, pass_name, res):
    file_name = f"{name}_{pass_name}_bake.png"
    img = bpy.data.images.new(file_name, res, res, float_buffer=True)
    mat = create_tex_bake_node(ob, file_name, img)
    bake(pass_name, "IMAGE_TEXTURES")

    path = os.path.join(basedir, file_name)
    save_image_use_saved(img, path)

    return mat, file_name, img

def consolidate_materials(ob):
    mat = ob.data.materials[0].copy()
    mat.name = ob.name
    ob.data.materials.clear()
    ob.data.materials.append(mat)
    return mat

def shader_emit(mat, color):
    links = mat.node_tree.links
    nodes = mat.node_tree.nodes
    emit_node = nodes.get("BAKE_EMISSION_OUTPUT")
    if emit_node is None:
        emit_node = nodes.new('ShaderNodeEmission')
        emit_node.name = 'BAKE_EMISSION_OUTPUT'
    links.new(color, emit_node.inputs[0])
    links.new(emit_node.outputs["Emission"], nodes.get("Material Output").inputs[0])

def shader_emit_vert(mat):
    links = mat.node_tree.links
    nodes = mat.node_tree.nodes

    attribute_node = nodes.get("BAKE_ATTRIBUTE")
    if attribute_node is None:
        attribute_node = nodes.new('ShaderNodeAttribute')
        attribute_node.attribute_name = "Bake"
        attribute_node.name = "BAKE_ATTRIBUTE"

    emit_node = nodes.get("BAKE_EMISSION_OUTPUT")
    if emit_node is None:
        emit_node = nodes.new('ShaderNodeEmission')
        emit_node.name = 'BAKE_EMISSION_OUTPUT'

    links.new(attribute_node.outputs["Color"], emit_node.inputs[0])
    links.new(emit_node.outputs["Emission"], nodes.get("Material Output").inputs[0])

def join_deselect(active, others, vlayer):
    deselect()
    for ob in others:
        ob.select_set(True)
    active.select_set(True)
    vlayer.objects.active = active
    bpy.ops.object.join()
    deselect()

def get_all_ob_in_col_recursive(collection):
    objects = []
    for ob in collection.objects:
        if ob.type != "MESH" or not ob.visible_get():
            continue
        objects.append(ob)
    for collection in collection.children_recursive:
        for ob in collection.objects:
            if ob.type != "MESH" or not ob.visible_get():
                continue
            objects.append(ob)
    return objects

def join_unwrap(collection, root_col_name):
    if "JOIN" in root_col_name:
        objects = get_all_ob_in_col_recursive(collection)
        if len(objects) > 1:
            join_deselect(objects[0], 
                          objects[1:], 
                          vlayer)  

    if "UNWRAP" in root_col_name:
        objects = get_all_ob_in_col_recursive(collection)
        if len(objects) > 1:
            smart_project_deselect(objects[0], 
                                   objects[1:], 
                                   vlayer, 
                                   island_margin=0.005)
        if len(objects) > 0:
            objects[0].select_set(True)
            vlayer.objects.active = objects[0]
            smart_project(island_margin=0.005)

def apply_collection(collection, vlayer):        
    for ob in get_all_ob_in_col_recursive(collection):
        apply_modifiers_scale_deselect(ob)

def process_collection(collection, root_col_name, vlayer):
    join_unwrap(collection, collection.name)

    for ob in collection.objects:  
        if ob.type != "MESH" or not ob.visible_get():
            continue
        print("PROC", ob.name)

        ob.select_set(True)

        create_material_if_needed(ob)


        if "VERT" in root_col_name:
            scene.cycles.samples = VERT_SAMPLES
            scene.view_layers[0].cycles.use_denoising = DENOISE_VERT

            setup_vertex_bake(ob)
            if SPLIT_ANYTHING:
                setup_edge_split(ob)
            apply_modifiers_scale_deselect(ob)

            ob.select_set(True)
            vlayer.objects.active = ob

            bake("COMBINED", "VERTEX_COLORS")
            if VERT_COL_METHOD != VERT_COL_CONVERT_TO:
                bpy.ops.geometry.attribute_convert(mode='GENERIC', domain=VERT_COL_CONVERT_TO, data_type='BYTE_COLOR')


        if "TEX" in root_col_name:

            resolution = 1024
            # double because of bake_proc_median
            if "256" in root_col_name:
                resolution = 512
            elif "512" in root_col_name:
                resolution = 1024
            elif "1024" in root_col_name:
                resolution = 2048
            elif "2048" in root_col_name:
                resolution = 4096
            resolution = int(resolution * RES_MULT)
            scene.cycles.samples = TEX_SAMPLES
            scene.view_layers[0].cycles.use_denoising = DENOISE_TEX
            mat, node, img = bake_pass_tex(ob, basedir, name, 
                                       "COMBINED", resolution)
            if MEDIAN_FILTER:
                bake_proc_median(img)

            mat = consolidate_materials(ob)
            nodes = mat.node_tree.nodes
            shader_emit(mat, nodes.get(node).outputs["Color"])

def full_process(root_collection, vlayer):
    apply_collection(root_collection, vlayer)
    join_unwrap(root_collection, root_collection.name)
    process_collection(root_collection, root_collection.name, vlayer)
    for collection in root_collection.children_recursive:
        apply_collection(collection, vlayer)
        join_unwrap(collection, collection.name)
        process_collection(collection, root_collection.name, vlayer)


def preview_vert_color_permanent(collection):
    for ob in collection.objects:  
        if ob.type != "MESH" or not ob.visible_get():
            continue
        mat = consolidate_materials(ob)
        shader_emit_vert(mat)

context = bpy.context

vlayer = context.view_layer
scene = context.scene
# export to blend file location
basedir = os.path.dirname(bpy.data.filepath)
if not basedir:
    raise Exception("Blend file is not saved")

# do props first
for name, root_collection in bpy.data.collections.items():
    if "PROPS" in root_collection.name:
        full_process(root_collection, vlayer)

for name, root_collection in bpy.data.collections.items():
    if "ARCH" in root_collection.name:
        full_process(root_collection, vlayer)

if PREVIEW_MODE:
    for name, root_collection in bpy.data.collections.items():
        if "PROPS" in root_collection.name and "VERT" in root_collection.name:
            preview_vert_color_permanent(root_collection)  
            for collection in root_collection.children_recursive:
                preview_vert_color_permanent(collection)  
