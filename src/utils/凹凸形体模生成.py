import numpy as np
from matplotlib.path import Path
from scipy.spatial import Delaunay

contour_b = np.array([
    [150, 120],
    [105, 184],
    [115, 188],
    [118, 188],
    [125, 191],
    [136, 193],
    [151, 198],
    [165, 205],
    [186, 221],
    [190, 226],
    [192, 226],
    [234, 187],
    [238, 182],
    [205, 140],
    [200, 137],
    [186, 132],
    [171, 129],
    [159, 125],
    # [150, 120]
])
contour_a = np.array([
    [269, 56],
    [239, 64],
    [191, 85],
    [168, 91],
    [126, 94],
    [84, 89],
    [59, 231],
    [158, 265],
    [187, 259],
    [249, 233],
    [268, 231],
    # [269, 56],
])
height_a = 100
height_b = 70

# 创建obj文件
title_content = f"mtllib Building1.mtl\no Building1"
contour_a_length = len(contour_a)
contour_b_length = len(contour_b)

# v点
v_content = ""
for point in contour_a:
    lon = point[0]
    lat = point[1]
    v_content += f"\nv {lon} {height_a} {lat}"

for point in contour_a:
    lon = point[0]
    lat = point[1]
    v_content += f"\nv {lon} 0 {lat}"

for point in contour_b:
    lon = point[0]
    lat = point[1]
    v_content += f"\nv {lon} {height_a} {lat}"

for point in contour_b:
    lon = point[0]
    lat = point[1]
    v_content += f"\nv {lon} {height_b} {lat}"

# print(v_content)

# 贴图
vt_content = "\nvt 0.625000 0.500000"
# 法向量
vn_content = "\nvn 0 1 0\nvn 0 -1 0"

# 外轮廓侧面
for i in range(1, contour_a_length + 1):
    point1 = contour_a[i - 1]
    point2 = contour_a[i - 1]
    point3 = contour_a[i] if i < contour_a_length else contour_a[0]
    a = (0 - height_a) * (point3[1] - point1[1]) - (0 - height_a) * (
            point2[1] - point1[1])
    b = 0
    c = (point2[0] - point1[0]) * (0 - height_a) - (
            point3[0] - point1[0]) * (0 - height_a)
    vn_content += f"\nvn {a} {b} {c}"

for i in range(1, contour_b_length + 1):
    point1 = contour_b[i - 1]
    point2 = contour_b[i - 1]
    point3 = contour_b[i] if i < contour_b_length else contour_b[0]
    a = (0 - height_b) * (point3[1] - point1[1]) - (0 - height_b) * (
            point2[1] - point1[1])
    b = 0
    c = (point2[0] - point1[0]) * (0 - height_b) - (
            point3[0] - point1[0]) * (0 - height_b)
    vn_content += f"\nvn {-a} {-b} {-c}"

# print(vn_content)

# 生成面组
face_content = "\ng box_Cube\nusemtl Material01\ns off"
# 生成侧面
side_face_content = ""
for i in range(1, contour_a_length + 1):
    side_face_content += "\nf "
    if i < contour_a_length:
        side_face_content += f"{i}/1/{i + 2} {i + contour_a_length}/1/{i + 2} {i + contour_a_length + 1}/1/{i + 2} {i + 1}/1/{i + 2}"
    else:
        side_face_content += f"{i}/1/{i + 2} {i + contour_a_length}/1/{i + 2} {1 + contour_a_length}/1/{i + 2} 1/1/{i + 2}"

for i in range(1, contour_b_length + 1):
    side_face_content += "\nf "
    offset = 2 * contour_a_length
    side_offset = contour_a_length
    if i < contour_b_length:
        side_face_content += f"{i + 1 + offset}/1/{i + 2 + side_offset} {i + contour_b_length + 1 + offset}/1/{i + 2 + side_offset} {i + contour_b_length + offset}/1/{i + 2 + side_offset} {i + offset}/1/{i + 2 + side_offset}"
    else:
        side_face_content += f"{1 + offset}/1/{i + 2 + side_offset} {1 + contour_b_length + offset}/1/{i + 2 + side_offset} {i + contour_b_length + offset}/1/{i + 2 + side_offset} {i + offset}/1/{i + 2 + side_offset} "
# 顶面
top_face_content = ""
# 底面
bottom_face_content = ""
contour_a = np.vstack([contour_a, contour_a[0]])
contour_b = np.vstack([contour_b, contour_b[0]])
new_points = np.concatenate([contour_a, contour_b])
new_points_length = len(new_points)

# 计算Delaunay三角剖分
tri = Delaunay(new_points)
# 创建一个路径对象，包含轮廓的全部点
contour_path = Path(contour_b)
contour_path2 = Path(new_points)
contour_path3 = Path(contour_a)
# 找到所有三角形的中点
tri_centers = np.mean(new_points[tri.simplices], axis=1)

# 确定中点是否在小轮廓路径内部
is_inside = contour_path.contains_points(tri_centers)
# 确定中点是否在大轮廓路径内部
is_inside2 = contour_path2.contains_points(tri_centers)

# 过滤掉在小轮廓内部的三角形与大轮廓外部的三角形
tri_simplices = tri.simplices[~is_inside * is_inside2]

triangles = new_points[tri_simplices]


def find_pos(substring, vcontent):
    # 找到点的原地点的部分
    v_arr = vcontent.split('\nv ')
    for i, v_item in enumerate(v_arr):
        if substring == v_item:
            return i
    return -1


for tri in triangles:
    top_face_content += "\nf "
    tri_reverse = np.flipud(tri)
    for point in tri_reverse:
        substring = f"{point[0]} {height_a} {point[1]}"
        num = find_pos(substring, v_content)
        if num != -1:
            top_face_content += f"{num}/1/1 "
        else:
            print("构造三角形的时候，出现了不存在的点")

# print(top_face_content)

tri_b = Delaunay(contour_b)
tri_b_centers = np.mean(contour_b[tri_b.simplices], axis=1)
is_inside_b = contour_path.contains_points(tri_b_centers)
tri_b_simplices = tri_b.simplices[is_inside_b]
triangles_b = contour_b[tri_b_simplices]
for tri in triangles_b:
    top_face_content += "\nf "
    tri_reverse = np.flipud(tri)
    for point in tri_reverse:
        substring = f"{point[0]} {height_b} {point[1]}"
        num = find_pos(substring, v_content)
        if num != -1:
            # print(num)
            # print(substring)
            top_face_content += f"{num}/1/1 "
        else:
            print("构造三角形的时候，出现了不存在的点")

tri_a = Delaunay(contour_a)
tri_a_centers = np.mean(contour_a[tri_a.simplices], axis=1)
is_inside_a = contour_path3.contains_points(tri_a_centers)
tri_a_simplices = tri_a.simplices[is_inside_a]
triangles_a = contour_a[tri_a_simplices]
for tri in triangles_a:
    bottom_face_content += "\nf "
    for point in tri:
        substring = f"{point[0]} 0 {point[1]}"
        num = find_pos(substring, v_content)
        if num != -1:
            bottom_face_content += f"{num}/1/2 "
        else:
            print("构造三角形的时候，出现了不存在的点")


obj_content = f"{title_content}{v_content}{vt_content}{vn_content}{face_content}{top_face_content}{bottom_face_content}{side_face_content}"
print(obj_content)

with open(f"Building1-模型.obj", "w") as obj_file:
    obj_file.write(obj_content)

