#%%
import json
import math
import os

import cv2
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.patches import Polygon


#三角剖分和模型生成的部分
#——————————————————————————————————————————————————
def plot_polygon(ax, polygon_data, color):
    # Extract lon and lat coordinates from polygon data
    lon_coords = [point['lon'] for point in polygon_data]
    lat_coords = [point['lat'] for point in polygon_data]
    polygon = Polygon(list(zip(lon_coords, lat_coords)), closed=True, fill=False, edgecolor=color)
    ax.add_patch(polygon)

def plot_triangle(ax, triangle_data, color):
    # Extract lon and lat coordinates from triangle data
    lon_coords = [triangle_data[key]['lon'] for key in ('p1', 'p2', 'p3')]
    lat_coords = [triangle_data[key]['lat'] for key in ('p1', 'p2', 'p3')]
    triangle = Polygon(list(zip(lon_coords, lat_coords)), closed=True, fill=False, edgecolor=color)
    ax.add_patch(triangle)

def point_cross_triangle_border(p0, p1, p2):
    # 判断p0为起始点，y轴向下方向的射线，是否与p1，p2线段相交
    cross_point_count = 0

    if p1['lon'] < p2['lon']:
        min_lon, max_lon = p1['lon'], p2['lon']
    else:
        min_lon, max_lon = p2['lon'], p1['lon']

    if p0['lon'] >= min_lon and p0['lon'] <= max_lon:
        # p0的x在p1和p2之间才有意义
        # 判断p1，p2的x是否一样
        if p1['lon'] == p2['lon']:
            # p1p2是一条垂直线
            # 不用检查p0是否在p1p2上
            cross_point_count += 0
        else:
            # p1p2不是垂直线，找到p1p2的方程式
            k = (p2['lat'] - p1['lat']) / (p2['lon'] - p1['lon'])
            b = p2['lat'] - k * p2['lon']
            crosspoint_y = p0['lon'] * k + b
            if p0['lat'] > crosspoint_y:
                cross_point_count += 1

    return cross_point_count


def check_point(i, p2, points):
    p1 = None
    p3 = None
    length = len(points)
    
    if i == 1:
        p1 = points[length - 1]
        p3 = points[i]
    elif i == length:
        p1 = points[i - 2]
        p3 = points[0]
    else:
        p1 = points[i - 2]
        p3 = points[i]

    v1 = {
        'x': p2['lon'] - p1['lon'],
        'y': p2['lat'] - p1['lat'],
        'z': 0
    }
    v2 = {
        'x': p3['lon'] - p2['lon'],
        'y': p3['lat'] - p2['lat'],
        'z': 0
    }

    z = v1['x'] * v2['y'] - v1['y'] * v2['x']

    if z < 0:
        print(f"点:({p2['lon']}, {p2['lat']})是凸点！")
        return True
    elif z == 0:
        print(f"点:({p2['lon']}, {p2['lat']})是平点！")
        return True
    else:
        print(f"点:({p2['lon']}, {p2['lat']})是凹点！")
        return False

def is_point_inside_triangle(p0, p1, p2, p3):
    def sign(p1, p2, p3):
        return (p1["lon"] - p3["lon"]) * (p2["lat"] - p3["lat"]) - (p2["lon"] - p3["lon"]) * (p1["lat"] - p3["lat"])

    d1 = sign(p0, p1, p2)
    d2 = sign(p0, p2, p3)
    d3 = sign(p0, p3, p1)

    has_neg = (d1 < 0) or (d2 < 0) or (d3 < 0)
    has_pos = (d1 > 0) or (d2 > 0) or (d3 > 0)

    return not (has_neg and has_pos)

def cut_polygon(points):
    # 会多次调用的图形切割方法
    # points为导入的点
    # 先判断这个图形是不是凸多边形
    is_convex = True
    convex_points = []  # 凸点数组
    for i in range(1, len(points) + 1):
        p2 = points[i - 1]

        if not check_point(i, p2, points):
            is_convex = False
        else:
            convex_points.append(i)

    result = {
        'convex': [],
        'triangles': []
    }

    if is_convex:
        # 是凸多边形，返回自身的点
        result['convex'] = points
        print("是凸多边形")
        return result
    else:
        # -----------------------
        # 是凹多边形，要递归切割
        print("是凹多边形")
        # -----------------------
        # 遍历凸点，找到一个能切割的点
        for point in convex_points:
            p1 = None
            p2 = None
            p3 = None
            length = len(points)
            p1_pos = -1  # 当前凸点及其两旁的点，在图形中的位置
            p2_pos = -1
            p3_pos = -1
            if point == 1:
                p1 = points[length - 1]
                p2 = points[0]
                p3 = points[1]
                p1_pos = length - 1
                p2_pos = 0
                p3_pos = 1
            elif point == length:
                p1 = points[length - 2]
                p2 = points[length - 1]
                p3 = points[0]
                p1_pos = length - 2
                p2_pos = length - 1
                p3_pos = 0
            else:
                p1 = points[point - 2]
                p2 = points[point - 1]
                p3 = points[point]
                p1_pos = point - 2
                p2_pos = point - 1
                p3_pos = point

            # 遍历points中除去p1、p2、p3的点，是否在p1p2p3的三角形内
            conflict = False
            for j in range(length):
                if j != p1_pos and j != p2_pos and j != p3_pos:
                    # 遍历到非p1p2p3的点的时候
                    # 把这个点从points中拿出来，检测在不在p1p2p3的三角形内
                    if is_point_inside_triangle(points[j], p1, p2, p3):
                        conflict = True

            # 遍历结束，查看冲突值，如果为true，则发生冲突，该点不能切割
            if conflict:
                print(p2, " 不是可划分点")
            else:
                print(p2, " 是可划分点")
                # 开始划分
                # 切割三角形出来
                tri_list = {
                    'p1': p1,
                    'p2': p2,
                    'p3': p3
                }
                # 切割新的多边形出来
                new_points = points.copy()
                new_points.pop(p2_pos)
                result2 = cut_polygon(new_points)
                result['convex'] = result2['convex']
                result['triangles'] = result2['triangles'].copy()
                result['triangles'].append(tri_list)
                # 停止循环
                break

        return result

def find_pos(substring, vcontent):
    # 找到点的原地点的部分
    v_arr = vcontent.split('\nv ')
    for i, v_item in enumerate(v_arr):
        if substring == v_item:
            return i
    return -1


def read_data(json_data):
    for info in json_data['buildingArray']:
        # 创建obj文件
        title_content = f"mtllib {info['name']}.mtl\no {info['name']}"
        points = info['points']
        points_length = len(points)

        if info['pointsOrder'] == "reverse":
            # 逆时针排序，改为顺时针
            points.reverse()

        # v点
        v_content = ""
        # 获取基准点
        basic_point = info['center'] if 'center' in info else {'lon': points[0]['lon'], 'lat': points[0]['lat']}
        for point in points:
            new_lon = point['lon'] - basic_point['lon']
            new_lat = point['lat'] - basic_point['lat']
            v_content += f"\nv {new_lon} {point['height']} {new_lat}"
        for point in points:
            new_lon = point['lon'] - basic_point['lon']
            new_lat = point['lat'] - basic_point['lat']
            point['lon'] = new_lon
            point['lat'] = new_lat
            v_content += f"\nv {new_lon} 0 {new_lat}"

        # 贴图
        vt_content = "\nvt 0.625000 0.500000"
        # 法向量
        vn_content = "\nvn 0 1 0\nvn 0 -1 0"
        # 侧面
        for i in range(1, points_length + 1):
            point1 = points[i - 1]
            point2 = points[i - 1]
            point3 = points[i] if i < points_length else points[0]
            a = (0 - point1['height']) * (point3['lat'] - point1['lat']) - (0 - point1['height']) * (
                    point2['lat'] - point1['lat'])
            b = 0
            c = (point2['lon'] - point1['lon']) * (0 - point1['height']) - (
                    point3['lon'] - point1['lon']) * (0 - point1['height'])
            vn_content += f"\nvn {a} {b} {c}"

        # 生成面组
        face_content = "\ng box_Cube\nusemtl Material01\ns off"
        # 生成侧面
        side_face_content = ""
        for i in range(1, points_length + 1):
            side_face_content += "\nf "
            if i < points_length:
                side_face_content += f"{i}/1/{i + 2} {i + points_length}/1/{i + 2} {i + points_length + 1}/1/{i + 2} {i + 1}/1/{i + 2}"
            else:
                side_face_content += f"{i}/1/{i + 2} {i + points_length}/1/{i + 2} {1 + points_length}/1/{i + 2} 1/1/{i + 2}"

        # 生成顶面和底面（new！！）
        # 顶面
        top_face_content = ""
        # 底面
        bottom_face_content = ""

        # 获取切割结果
        cut_face = cut_polygon(points)
        print("三角切割结果")
        print(cut_face)
        lon_coords = [point['lon'] for point in cut_face['convex']]
        lat_coords = [point['lat'] for point in cut_face['convex']]
        lon_coords_tri = [triangle_data[key]['lon'] for triangle_data in cut_face['triangles'] for key in ('p1', 'p2', 'p3')]
        lat_coords_tri = [triangle_data[key]['lat'] for triangle_data in cut_face['triangles'] for key in ('p1', 'p2', 'p3')]

        fig, ax = plt.subplots()

        # Plot convex polygon
        plot_polygon(ax, cut_face['convex'], color='blue')

        # Plot triangles
        for triangle_data in cut_face['triangles']:
            plot_triangle(ax, triangle_data, color='red')

        # Set axis limits and labels
        ax.set_xlim(min(lon_coords + lon_coords_tri) - 20, max(lon_coords + lon_coords_tri) + 20)
        ax.set_ylim(min(lat_coords + lat_coords_tri) - 20, max(lat_coords + lat_coords_tri) + 20)
        ax.set_xlabel('Lon')
        ax.set_ylabel('Lat')
        ax.set_aspect('equal')
        ax.invert_yaxis()

        # Add legend
        # legend_elements = [Line2D([0], [0], color='blue', lw=2, label='Convex Polygon'),
        #                    Line2D([0], [0], color='red', lw=2, label='Triangle')]
        # ax.legend(handles=legend_elements)

        # 绘制三角剖分的结果
        # plt.show()

        # 这里还有点问题，要改
        # 已经处理了\nf 出现的问题
        # 生成切割后的凸多边形
        success_convex = False
        convex = cut_face['convex']
        for point in convex:
            substring = f"{point['lon']} {point['height']} {point['lat']}"
            num = find_pos(substring, v_content)
            if num != -1:
                top_face_content += f"{num}/1/1 "
                bottom_face_content = f"{num + points_length}/1/2 {bottom_face_content}"
                success_convex = True
            else:
                print("构造凸多边形的时候，出现了不存在的点")

        if success_convex:
            top_face_content = f"\nf {top_face_content}"
            bottom_face_content = f"\nf {bottom_face_content}"

        # 生成切割后的多个三角形
        triangles = cut_face['triangles']
        for tri in triangles:
            top_face_content += "\nf "
            for point in tri.values():
                substring = f"{point['lon']} {point['height']} {point['lat']}"
                num = find_pos(substring, v_content)
                if num != -1:
                    top_face_content += f"{num}/1/1 "
                    bottom_face_content = f"{num + points_length}/1/2 {bottom_face_content}"
                else:
                    print("构造三角形的时候，出现了不存在的点")
            bottom_face_content = f"\nf {bottom_face_content}"

        # 这里top_face_content和bottom_face_content添加"\n f"的方式不同是因为顶面和底面的点顺序不同
        # print('top_face_content',top_face_content);
        # print('bottom_face_content',bottom_face_content);
        # print('side_face_content',bottom_face_content)

        # obj文件统合
        obj_content = f"{title_content}{v_content}{vt_content}{vn_content}{face_content}{top_face_content}{bottom_face_content}{side_face_content}"
        # 创建mtl文件
        mtl_content = "newmtl Material01\nNs 50\nKa 1 1 1\nKd 0.800000 0.269435 0.285941\nKs 0.664835 0.664835 0.664835\nKe 0 0 0\nNi 1.450000\nd 1.000000\nillum 2"

        # 打印结果
        # print(obj_content)
        # print(mtl_content)

        # Python的文件生成方式
        with open(f"{info['name']}-模型.obj", "w") as obj_file:
            obj_file.write(obj_content)

        # with open(f"{info['name']}.mtl", "w") as mtl_file:
        #     mtl_file.write(mtl_content)



#轮廓信息提取部分
#——————————————————————————————————————————————————
# 定义一个类来表示点对象
class Point:
    def __init__(self, lon, lat):
        self.lon = lon
        self.lat = lat

def find_contour_outline(image):
    """
    找到图像外轮廓的点集，并按顺序连接成封闭轮廓

    参数：
    image：一个二维的numpy数组，代表图像。

    返回值：
    contour：一个numpy数组，表示轮廓点集，点按顺序连接可以还原图形的边界。
    """

    # 找到二值图像的轮廓和层级结构
    contours, hierarchy = cv2.findContours(image, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
    #print(contours)
    # 找到图像最外层轮廓的索引
    max_contour_idx = np.argmax([cv2.contourArea(cnt) for cnt in contours])
    #print(max_contour_idx)
    # 获取最外层轮廓的点集
    contour = contours[max_contour_idx]
    print("原始轮廓与点集数量：")
#     print(contour)
    print(len(contour))

    # 根据轮廓面积降采样点集，保证点数量最少
    perimeter = cv2.arcLength(contour, True)
    epsilon = 0.002 * perimeter
    approx = cv2.approxPolyDP(contour, epsilon, True)

    # 将点集按顺序连接成封闭轮廓
    contour = approx.reshape(-1,2)
    contour = np.vstack([contour, contour[0]])

    return contour


# 全局变量
# 窗口名
# g_window_wh = [800, 600]  # 窗口宽高
#
# g_location_win = [0, 0]  # 相对于大图，窗口在图片中的位置
# location_win = [0, 0]  # 鼠标左键点击时，暂存g_location_win
# g_location_click, g_location_release = [0, 0], [0, 0]  # 相对于窗口，鼠标左键点击和释放的位置
#
# g_zoom, g_step = 1, 0.1  # 图片缩放比例和缩放系数
# g_image_original = cv2.imread("1.png")  # 原始图片，建议大于窗口宽高（800*600）
# g_image_zoom = g_image_original.copy()  # 缩放后的图片
# g_image_show = g_image_original[g_location_win[1]:g_location_win[1] + g_window_wh[1],
#                g_location_win[0]:g_location_win[0] + g_window_wh[0]]  # 实际显示的图片
#
#
# mask_original = np.zeros((g_image_original.shape[0] + 2, g_image_original.shape[1] + 2, 1), np.uint8)
#
# p = 0
# g_window_name = "contourImg"
g_window_wh = [912, 1230]  # 窗口宽高
g_location_win = [0, 0]  # 相对于大图，窗口在图片中的位置
g_zoom, g_step = 1, 0.1  # 图片缩放比例和缩放系数
g_image_original = None  # 原始图片
g_image_zoom = None  # 缩放后的图片
g_image_show = None  # 实际显示的图片
mask_original = None  # 用于图像处理的掩码
p = 0

# 矫正窗口在图片中的位置
# img_wh:图片的宽高, win_wh:窗口的宽高, win_xy:窗口在图片的位置
def check_location(img_wh, win_wh, win_xy):
    for i in range(2):
        if win_xy[i] < 0:
            win_xy[i] = 0
        elif win_xy[i] + win_wh[i] > img_wh[i] and img_wh[i] > win_wh[i]:
            win_xy[i] = img_wh[i] - win_wh[i]
        elif win_xy[i] + win_wh[i] > img_wh[i] and img_wh[i] < win_wh[i]:
            win_xy[i] = 0
    # print(img_wh, win_wh, win_xy)


# 计算缩放倍数
# flag：鼠标滚轮上移或下移的标识, step：缩放系数，滚轮每步缩放0.1, zoom：缩放倍数
def count_zoom(flag, step, zoom):
    if flag > 0:  # 滚轮上移
        zoom += step
        if zoom > 1 + step * 20:  # 最多只能放大到3倍
            zoom = 1 + step * 20
    else:  # 滚轮下移
        zoom -= step
        if zoom < step:  # 最多只能缩小到0.1倍
            zoom = step
            # print(zoom)
    zoom = round(zoom, 2)  # 取2位有效数字
    return zoom


global  g_window_name



def right_click(x, y):
    global g_image_zoom, g_image_original, mask_original, g_location_win,g_window_wh
    g_original_h, g_original_w = g_image_zoom.shape[0:2]
    mask = np.zeros((g_original_h + 2, g_original_w + 2, 1), np.uint8)
    mask[150:250, 150:250] = 0
    cv2.floodFill(g_image_zoom, mask, (g_location_win[0] + x, g_location_win[1] + y), (255, 0, 0), (30, 30, 30), (30, 30, 30), cv2.FLOODFILL_FIXED_RANGE)
    scale = g_image_zoom.shape[1] / g_image_original.shape[1]
    original_x = (g_location_win[0] + x) / scale
    original_y = (g_location_win[1] + y) / scale

    min_dist = float('inf')
    nearest_point = None
    for i in range(0 if 0 > math.floor(original_x) - 1 else math.floor(original_x) - 1, math.ceil(original_x) + 1 if math.ceil(original_x) + 1 < g_image_original.shape[1] else g_image_original.shape[1]):
        for j in range(0 if 0 > math.floor(original_y) - 1 else math.floor(original_y) - 1, math.ceil(original_y) + 1 if math.ceil(original_y) + 1 < g_image_original.shape[0] else g_image_original.shape[0]):
            dist = math.sqrt((original_x - i) ** 2 + (original_y - j) ** 2)
            if dist < min_dist:
                min_dist = dist
                nearest_point = (i, j)

    cv2.floodFill(g_image_original, mask_original, nearest_point, (255, 0, 0), (30, 30, 30),(30, 30, 30), cv2.FLOODFILL_FIXED_RANGE)
def process_image_from_json(json_data):
    global g_image_zoom, g_image_original, mask_original, g_location_win,g_window_wh
    # g_window_wh = [json_data['window']['width'], json_data['window']['height']]
    g_location_win = [json_data['location']['win_x'], json_data['location']['win_y']]
    # g_zoom = json_data['zoom']
    # g_step = json_data['step']
    click_location = json_data['right_click']
    image_path=json_data['image_path']
    g_image_original = cv2.imread(image_path)
    g_window_wh = [g_image_original.shape[1], g_image_original.shape[0]]
    # g_image_original = cv2.imread(json_data['image_path'])
    g_image_zoom=g_image_original.copy()
    # g_image_zoom = cv2.resize(g_image_original, (int(g_image_original.shape[1] * g_zoom), int(g_image_original.shape[0] * g_zoom)), interpolation=cv2.INTER_AREA)
    g_image_show = g_image_zoom[g_location_win[1]:g_location_win[1] + g_window_wh[1], g_location_win[0]:g_location_win[0] + g_window_wh[0]]
    g_window_name = "contourImg"

    # Create the window before resizing it
    cv2.namedWindow(g_window_name, cv2.WINDOW_NORMAL)
    cv2.resizeWindow(g_window_name, g_window_wh[0], g_window_wh[1])

    # Call the right click function with parameters
    right_click(click_location["x"], click_location["y"])
    # Save the processed image
    cv2.imwrite("floodfill.png", g_image_zoom)

contourn_points=[]

def main(image_path, json_path):
    current_dir = os.path.dirname(__file__)
    global g_image_zoom, g_image_original, mask_original, g_location_win,contourn_points,g_window_wh
    # 构建相对于当前文件的路径
    # image_path = os.path.join(current_dir, image_path)
    # json_path= os.path.join(current_dir, json_path)

    # g_window_wh = [800, 600]  # 窗口宽高
    # g_zoom, g_step = 1, 0.1  # 图片缩放比例和缩放系数
    g_image_original = cv2.imread(image_path)  # 原始图片，建议大于窗口宽高（800*600）
    # 读取 JSON 文件
    with open(json_path, 'r') as f:
        json_data = json.load(f)

    # 调用 process_image_from_json 函数
    # process_image_from_json(json_data)

    # g_window_wh = [json_data['window']['width'], json_data['window']['height']]
    g_location_win = [json_data['location']['win_x'], json_data['location']['win_y']]
    # g_zoom = json_data['zoom']
    # g_step = json_data['step']

    process_image(json_data)

    # click_location = json_data['right_click']
    # image_path=json_data['image_path']
    # g_image_original = cv2.imread(image_path)
    # # g_image_original = cv2.imread(json_data['image_path'])
    # g_image_zoom=g_image_original.copy()
    # # g_image_zoom = cv2.resize(g_image_original, (int(g_image_original.shape[1] * g_zoom), int(g_image_original.shape[0] * g_zoom)), interpolation=cv2.INTER_AREA)
    # g_image_show = g_image_zoom[g_location_win[1]:g_location_win[1] + g_window_wh[1], g_location_win[0]:g_location_win[0] + g_window_wh[0]]
    # g_window_name = "contourImg"
    #
    # # Create the window before resizing it
    # cv2.namedWindow(g_window_name, cv2.WINDOW_NORMAL)
    # cv2.resizeWindow(g_window_name, g_window_wh[0], g_window_wh[1])
    #
    # # Call the right click function with parameters
    # right_click(click_location["x"], click_location["y"])
    # # Save the processed image
    # cv2.imwrite("floodfill.png", g_image_zoom)
    #
    # if g_image_original is None:
    #     raise FileNotFoundError(f"Image at path {image_path} not found or could not be loaded.")
    #
    # g_image_zoom = g_image_original.copy()  # 缩放后的图片
    # g_location_win = [0, 0]  # 相对于大图，窗口在图片中的位置
    #
    # g_image_show = g_image_original[g_location_win[1]:g_location_win[1] + g_window_wh[1],
    #                g_location_win[0]:g_location_win[0] + g_window_wh[0]]  # 实际显示的图片
    #
    # # 保存结果
    # cv2.imwrite('floodfill.png', g_image_zoom)
    #
    # # 提取特定颜色像素点
    # mask = cv2.inRange(g_image_original, (255, 0, 0), (255, 0, 0))
    #
    # # 将特定颜色像素点变为白色，其余像素点变为黑色
    # g_image_original[mask == 255] = [255, 255, 255]
    # g_image_original[mask != 255] = [0, 0, 0]
    #
    # # 保存结果
    # cv2.imwrite('test.png', g_image_original)
    #
    # image = cv2.imread("test.png")
    # gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    #
    # # 膨胀操作
    # kernel = np.ones((5, 5), np.uint8)
    # dilation = cv2.dilate(gray, kernel, iterations=1)
    # dilation = cv2.erode(dilation, kernel, iterations=1)
    #
    # dilation_color = np.ones_like(image) * 255
    # dilation_color[dilation == 255] = [200, 150, 150]
    #
    # cv2.imwrite('dilation.png', dilation)
    # cv2.imwrite('dilation_color.png', dilation_color)
    #
    # image = cv2.imread("dilation.png")
    # gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    #
    # ret, image = cv2.threshold(gray, 127, 255, 0)
    #
    # contour = find_contour_outline(image)
    # print("失真程度为0.002*周长以内的点结果")
    # for i in range(len(contour)):
    #     print('[' + str(contour[i][0]) + ',' + str(contour[i][1]) + '],')
    # print(len(contour))
    # middle_part = contour[:-1]
    # n = 16
    # n %= len(middle_part)
    # contour = np.vstack([middle_part[n:], middle_part[:n]])
    # contour = np.vstack([contour, contour[0]])
    #
    # points = [Point(lon, lat).__dict__ for lon, lat in contour]
    # points = points[:-1]
    # for point in points:
    #     point['height'] = 100
    #
    # contour_points=points
    #
    # json_data_example = {
    #     "buildingArray": [
    #         {
    #             "name": "Building1",
    #             "points": [],
    #             "pointsOrder": "order",
    #             "center": {"lon": 0, "lat": 0}
    #         }
    #     ]
    # }
    # json_data_example["buildingArray"][0]["points"] = points
    # print(json_data_example)
    #
    #
    # with open('outputtest.json', 'w') as outfile:
    #     json.dump(json_data_example, outfile, cls=CustomEncoder)
    # read_data(json_data_example)
    
class CustomEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, np.integer):
            return int(obj)
        return super().default(obj)

def process_image(json_data):
    contourn_points = {
        "parent": {
            "contour_points": [],
            "height": 100
        },
        "children": []
    }
    global g_image_zoom, g_image_original, mask_original, g_location_win
    click_locations = json_data['right_clicks']
    image_path=json_data['image_path']
    # g_image_original = cv2.imread(image_path)
    g_image_original = cv2.imread(json_data['image_path'])
    g_image_zoom=g_image_original.copy()
    # g_image_zoom = cv2.resize(g_image_original, (int(g_image_original.shape[1] * g_zoom), int(g_image_original.shape[0] * g_zoom)), interpolation=cv2.INTER_AREA)
    # g_image_show = g_image_zoom[g_location_win[1]:g_location_win[1] + g_window_wh[1], g_location_win[0]:g_location_win[0] + g_window_wh[0]]
    g_window_name = "contourImg"

    # Create the window before resizing it
    cv2.namedWindow(g_window_name, cv2.WINDOW_NORMAL)
    cv2.resizeWindow(g_window_name, g_window_wh[0], g_window_wh[1])
    with open('newoutput2.json', 'w') as outfile:
        json.dump(json_data, outfile, cls=CustomEncoder)

    for click_location in click_locations:
    # Call the right click function with parameters
        g_image_original = cv2.imread(json_data['image_path'])
        g_image_zoom=g_image_original.copy()
        right_click(click_location["x"], click_location["y"])
        # 每次处理都重新加载原始图片

        # g_image_zoom=g_image_original.copy()
        # Save the processed image
        # cv2.imwrite("floodfill.png", g_image_zoom)

        if g_image_original is None:
            raise FileNotFoundError(f"Image at path {image_path} not found or could not be loaded.")

        # g_image_zoom = g_image_original.copy()  # 缩放后的图片
        g_location_win = [0, 0]  # 相对于大图，窗口在图片中的位置

        g_image_show = g_image_original[g_location_win[1]:g_location_win[1] + g_window_wh[1],
                       g_location_win[0]:g_location_win[0] + g_window_wh[0]]  # 实际显示的图片

        # 保存结果
        # cv2.imwrite('floodfill.png', g_image_zoom)

        # 提取特定颜色像素点
        mask = cv2.inRange(g_image_original, (255, 0, 0), (255, 0, 0))

        # 将特定颜色像素点变为白色，其余像素点变为黑色
        g_image_original[mask == 255] = [255, 255, 255]
        g_image_original[mask != 255] = [0, 0, 0]

        # 保存结果
        cv2.imwrite('test.png', g_image_original)

        image = cv2.imread("test.png")
        gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

        # 膨胀操作
        kernel = np.ones((5, 5), np.uint8)
        dilation = cv2.dilate(gray, kernel, iterations=1)
        dilation = cv2.erode(dilation, kernel, iterations=1)

        dilation_color = np.ones_like(image) * 255
        dilation_color[dilation == 255] = [200, 150, 150]

        cv2.imwrite('dilation.png', dilation)
        # cv2.imwrite('dilation_color.png', dilation_color)

        image = cv2.imread("dilation.png")
        gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

        ret, image = cv2.threshold(gray, 127, 255, 0)

        contour = find_contour_outline(image)
        print("失真程度为0.002*周长以内的点结果")
        for i in range(len(contour)):
            print('[' + str(contour[i][0]) + ',' + str(contour[i][1]) + '],')
        print(len(contour))
        middle_part = contour[:-1]
        n = 16
        n %= len(middle_part)
        contour = np.vstack([middle_part[n:], middle_part[:n]])
        contour = np.vstack([contour, contour[0]])

        points = [Point(lon, lat).__dict__ for lon, lat in contour]
        points = points[:-1]
        for point in points:
            point['height'] = 100

        if click_location['type'] == 'parent':
            contourn_points['parent']['contour_points'] = points

        elif click_location['type'] == 'children':
            contourn_points['children'].append({
                'contour_points': points,
                'height': 100
            })
    with open('newoutput2.json', 'w') as outfile:
        json.dump(contourn_points, outfile, cls=CustomEncoder,indent=4)




def process_children_parent_right_clicks(json_data):
    for click in json_data['right_clicks']:
        if click['type'] == 'parent':
            right_click(click['x'], click['y'])
        elif click['type'] == 'children':
            right_click(click['x'], click['y'])

if __name__ == "__main__":
    
    json_path = 'input2.json'
    with open('input2.json', 'r') as f:
        json_data=json.load(f)

    image_path=json_data['image_path']
    main(image_path, json_path)
      # 将 contourn_points 序列化并保存为 JSON 文件
    with open('contourn_points.json', 'w') as outfile:
        json.dump(contourn_points, outfile, cls=CustomEncoder)
    
    

