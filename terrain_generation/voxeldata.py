import numpy as np
from PIL import Image
import struct

if __name__ == "__main__":
    # path = "/mnt/c/screenshots_and_videos/wanderer head shot.PNG"
    path = "/mnt/c/screenshots_and_videos/wanderer head shot.PNG"
    image = Image.open(path)
    a = np.array(image)

    vd = np.zeros((15, a.shape[0], a.shape[1], 4), dtype=np.uint8)

    # intensities range from 0 to 15
    intensity = np.sum(a[:, :, :3], axis=2) * 5 / 255
    intensity1 = np.repeat(intensity[:, :, np.newaxis], 1, axis=2)
    intensity3 = np.repeat(intensity[:, :, np.newaxis], 3, axis=2)
    print(intensity1.shape)
    print(intensity3.shape)

    for i in range(15):
        vd[i, :, :, 1:] = np.where(intensity3 > i, a[:, :, :3], 0)
        vd[i, :, :, :1] = np.where(intensity1 > i, 1, 0)

    # np.shape is backwards from our coordinate convention, = (z, y, x, voxel)
    # np.array[z][y][x][voxel]

    with open("/mnt/c/screenshots_and_videos/wanderer_intensity.vd", "wb") as f:
        # > is big-endian
        b = struct.pack(">iii", vd.shape[2], vd.shape[1], vd.shape[0])
        f.write(b)
        for z in range(vd.shape[0]):
            for y in range(vd.shape[1]):
                for x in range(vd.shape[2]):
                    b = struct.pack(
                        ">ifff",
                        vd[z, y, x, 0],
                        vd[z, y, x, 1] / 255.0,
                        vd[z, y, x, 2] / 255.0,
                        vd[z, y, x, 3] / 255.0,
                    )
                    # print(
                    #     vd[z, y, x, 0],
                    #     vd[z, y, x, 1] / 255.0,
                    #     vd[z, y, x, 2] / 255.0,
                    #     vd[z, y, x, 3] / 255.0,
                    # )

                    f.write(b)
