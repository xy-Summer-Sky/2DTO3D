import numpy as np
from matplotlib.path import Path
from scipy.spatial import Delaunay
import matplotlib.pyplot as plt

title_content = f"mtllib Building1.mtl\no Building1"
# 顶点
v_content = ""
# 顶点索引
vertex2num_dict = {}
# 顶点计数
vertex_count = 0
# 贴图
vt_content = "\nvt 0.625000 0.500000"
# 法向量
vn_content = "\nvn 0 1 0\nvn 0 -1 0"
# 法向量计数
vnormal_count = 0
# 生成面组
face_content = "\ng box_Cube\nusemtl Material01\ns off"
# 生成侧面
side_face_content = ""
# 生成顶面
top_face_content = ""
# 生成底面
bottom_face_content = ""


def genContourVertex(contour, height):
    global vertex_count
    global v_content
    for point in contour:
        lon = point[0]
        lat = point[1]
        v_content += f"\nv {lon} {height} {lat}"
        vertex2num_dict[f'v {lon} {height} {lat}'] = vertex_count
        vertex_count += 1
    return None


def genSideFace(contour, height_self, height_target):
    global vn_content
    global vnormal_count
    global side_face_content
    for i in range(1, len(contour) + 1):
        point1 = contour[i - 1]
        point2 = contour[i - 1]
        point3 = contour[i] if i < len(contour) else contour[0]
        point4 = contour[i] if i < len(contour) else contour[0]
        a = (0 - height_self) * (point3[1] - point1[1]) - (0 - height_self) * (
                point2[1] - point1[1])
        b = 0
        c = (point2[0] - point1[0]) * (0 - height_self) - (
                point3[0] - point1[0]) * (0 - height_self)
        vn_content += f"\nvn {a} {b} {c}"
        vnormal_count += 1
        p1_substr = f"v {point1[0]} {height_self} {point1[1]}"
        p2_substr = f"v {point2[0]} {height_target} {point2[1]}"

        p3_substr = f"v {point3[0]} {height_target} {point3[1]}"
        p4_substr = f"v {point4[0]} {height_self} {point4[1]}"
        p1_num = vertex2num_dict[p1_substr]
        p2_num = vertex2num_dict[p2_substr]
        p3_num = vertex2num_dict[p3_substr]
        p4_num = vertex2num_dict[p4_substr]

        side_face_content += "\nf "
        side_face_content += f"{p1_num}/1/{vnormal_count + 2} {p2_num}/1/{vnormal_count + 2} {p3_num}/1/{vnormal_count + 2} {p4_num}/1/{vnormal_count + 2}"
    return None


def genTopFace(contour, child_contours, height):
    global top_face_content
    # 计算Delaunay三角剖分
    tri = Delaunay(contour)
    # 创建一个路径对象，包含轮廓的全部点
    contour_paths = []
    for child_contour in child_contours:
        contour_paths.append(Path(child_contour))
    contour_path2 = Path(contour)
    # 找到所有三角形的中点
    tri_centers = np.mean(contour[tri.simplices], axis=1)

    # 确定中点是否在小轮廓路径内部
    is_inside = contour_paths[0].contains_points(tri_centers)
    for contour_path in contour_paths:
        is_inside *= contour_path.contains_points(tri_centers)
    # 确定中点是否在大轮廓路径内部
    is_inside2 = contour_path2.contains_points(tri_centers)

    # 过滤掉在小轮廓内部的三角形与大轮廓外部的三角形
    tri_simplices = tri.simplices[~is_inside * is_inside2]
    triangles = contour[tri_simplices]
    for tri in triangles:
        top_face_content += "\nf "
        tri_reverse = np.flipud(tri)
        for point in tri_reverse:
            substring = f"v {point[0]} {height} {point[1]}"
            num = vertex2num_dict[substring]
            if num != -1:
                top_face_content += f"{num + 1}/1/1 "
            else:
                print("构造三角形的时候，出现了不存在的点")
    plt.figure(figsize=(10, 10), facecolor='lightgrey')

    # 绘制轮廓的三角剖分
    plt.triplot(contour[:, 0], contour[:, 1], tri_simplices, color='darkblue')
    # plt.triplot(new_points[:, 0], new_points[:, 1], tri.simplices.copy(), color='darkblue')

    # 绘制轮廓的点
    plt.plot(contour[:, 0], contour[:, 1], 'o', color='red', markersize=5)

    # 设置坐标轴的颜色和标签
    plt.xlabel('X')
    plt.ylabel('Y')
    plt.title('Constrained Delaunay Triangulation within Contour')

    # 反转Y轴
    plt.gca().invert_yaxis()

    # 显示图形
    plt.show()
    return None


def genTopFace2(contour, height):
    global top_face_content
    # print(contour)
    contour_path = Path(contour)
    tri_b = Delaunay(contour)
    tri_b_centers = np.mean(contour[tri_b.simplices], axis=1)
    is_inside_b = contour_path.contains_points(tri_b_centers)
    tri_b_simplices = tri_b.simplices[is_inside_b]
    triangles_b = contour[tri_b_simplices]
    # print(len(triangles_b))
    for tri in triangles_b:
        top_face_content += "\nf "
        tri_reverse = np.flipud(tri)
        for point in tri_reverse:
            substring = f"v {point[0]} {height} {point[1]}"
            # print(substring)
            num = vertex2num_dict[substring]
            if num != -1:
                # print(num)
                # print(substring)
                top_face_content += f"{num}/1/1 "
            else:
                print("构造三角形的时候，出现了不存在的点")


def genBottomFace(contour, height):
    global bottom_face_content
    contour_path = Path(contour)
    tri_a = Delaunay(contour)
    tri_a_centers = np.mean(contour[tri_a.simplices], axis=1)
    is_inside_a = contour_path.contains_points(tri_a_centers)
    tri_a_simplices = tri_a.simplices[is_inside_a]
    # triangles_a = contour_a[tri_a_simplices]
    triangles_a = contour[tri_a_simplices]
    for tri in triangles_a:
        bottom_face_content += "\nf "
        for point in tri:
            substring = f"v {point[0]} {height} {point[1]}"
            num = vertex2num_dict[substring]
            if num != -1:
                bottom_face_content += f"{num}/1/2 "
            else:
                print("构造三角形的时候，出现了不存在的点")
    return None


def generateModel(contour, parent_contour, child_contours, height_params):
    global v_content, vertex_count
    parent_height = height_params[0]
    genContourVertex(parent_contour, parent_height)
    genContourVertex(parent_contour, 0)
    for idx, child_contour in enumerate(child_contours):
        genContourVertex(child_contour, parent_height)
        genContourVertex(child_contour, height_params[idx + 1])
    print(vertex2num_dict)
    genSideFace(parent_contour, parent_height, 0)
    for idx, child_contour in enumerate(child_contours):
        genSideFace(child_contour, height_params[idx + 1], parent_height)

    genTopFace(contour, child_contours, height_params[0])
    for idx, child_contour in enumerate(child_contours):
        genTopFace2(child_contour, height_params[idx + 1])
    genBottomFace(parent_contour, 0)


# contour_b = np.array([
#     [155, 124],
#     [110, 188],
#     [112, 188],
#     [120, 192],
#     [123, 192],
#     [130, 195],
#     [141, 197],
#     [156, 202],
#     [170, 209],
#     [191, 225],
#     [195, 230],
#     [197, 230],
#     [239, 191],
#     [243, 186],
#     [210, 144],
#     [205, 141],
#     [191, 136],
#     [169, 131],
# ])
# contour_a = np.array([
#     [274, 60],
#     [244, 68],
#     [201, 87],
#     [173, 95],
#     [131, 98],
#     [89, 93],
#     [64, 234],
#     [163, 269],
#     [192, 263],
#     [254, 237],
#     [274, 234],
# ])
# contour_c = np.array([
#     [221, 93],
#     [221, 131],
#     [262, 131],
#     [262, 93],
# ])
# height_a = 100
# height_b = 120
# height_c = 120
# parent_height = height_a
# contour_a = np.vstack([contour_a, contour_a[0]])
# contour_b = np.vstack([contour_b, contour_b[0]])
# contour_c = np.vstack([contour_c, contour_c[0]])
# new_points = np.concatenate([contour_a, contour_b, contour_c])
# new_points_length = len(new_points)
#
# generateModel(new_points, contour_a, [contour_b, contour_c], [100, 70, 150])
# # print(vertex_count)
# obj_content = f"{title_content}{v_content}{vt_content}{vn_content}{face_content}{top_face_content}{bottom_face_content}{side_face_content}"
# print(obj_content)
# # print(top_face_content)
# with open(f"Building2-模型.obj", "w") as obj_file:
#     obj_file.write(obj_content)

def main(contour,parent_contour,child_contours,height_params,save_path):
    generateModel(contour, parent_contour, child_contours, height_params)
# print(vertex_count)
    obj_content = f"{title_content}{v_content}{vt_content}{vn_content}{face_content}{top_face_content}{bottom_face_content}{side_face_content}"
    print(obj_content)
# print(top_face_content)
    with open(f"Building2-模型.obj", "w") as obj_file:
        obj_file.write(obj_content)
