import json
import sys
import numpy as np
from scipy.spatial import Delaunay
import argparse

def create_delaunay(points):
    points = np.array(points)
    tri = Delaunay(points)
    return tri.simplices.tolist()

def main():
    parser = argparse.ArgumentParser(description='Generate model from contour points and heights.')
    parser.add_argument('height_a', type=float, help='Height A')
    parser.add_argument('height_b', type=float, help='Height B')
    parser.add_argument('height_c', type=float, help='Height C')
    parser.add_argument('contour_points', type=str, help='JSON encoded contour points')
    args = parser.parse_args()

    height_a = args.height_a
    height_b = args.height_b
    height_c = args.height_c
    contour_points = json.loads(args.contour_points)

    triangles = create_delaunay(contour_points)

    model = {
        "heights": {"A": height_a, "B": height_b, "C": height_c},
        "triangles": triangles
    }

    print(json.dumps(model))

if __name__ == "__main__":
    main()