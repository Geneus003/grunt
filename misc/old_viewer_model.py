import numpy as np
import json
import matplotlib.pyplot as plt

def main():
    model_file = open("../my_model.json")
    model_file = json.load(model_file)

    model = []
    for i, e in enumerate(model_file["model"]):
        model.append([])
        for j, ee in enumerate(e[f"depth_{i}"]):
            model[-1].append([])
            for k in ee[f"y_{j}"]:
                model[-1][-1].append(int(k))

    model = np.array(model)

    z, y, x = np.indices(np.array(model.shape) + 1).astype(int)

    ax = plt.figure().add_subplot(111,projection ='3d')
    colors = plt.cm.plasma(model)

    ax.voxels(x, y, z, model, facecolors=colors, alpha=0.8)
    ax.invert_zaxis()
    ax.set_xlabel("X_axis")
    ax.set_ylabel("Y_axis")
    ax.set_zlabel("Z_axis(depth)")
    plt.show()


if __name__ == "__main__":
    main()

