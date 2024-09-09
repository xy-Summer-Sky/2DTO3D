import cv2
import numpy as np
import base64
import sys
import json
import argparse

def find_contour_outline(image):
    contours, _ = cv2.findContours(image, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
    max_contour_idx = np.argmax([cv2.contourArea(cnt) for cnt in contours])
    contour = contours[max_contour_idx]
    perimeter = cv2.arcLength(contour, True)
    epsilon = 0.002 * perimeter
    approx = cv2.approxPolyDP(contour, epsilon, True)
    contour = approx.reshape(-1, 2)
    contour = np.vstack([contour, contour[0]])
    return contour

def main():
    parser = argparse.ArgumentParser(description='Extract contour from image.')
    parser.add_argument('image_data', type=str, help='Base64 encoded image data')
    args = parser.parse_args()

    image_data = base64.b64decode(args.image_data)
    nparr = np.frombuffer(image_data, np.uint8)
    img = cv2.imdecode(nparr, cv2.IMREAD_GRAYSCALE)
    contour = find_contour_outline(img)
    print(json.dumps(contour.tolist()))

if __name__ == "__main__":
    main()