import numpy as np
import json
import pyvista as pv

class ViewerEngine:
    def __init__(self, mesh, model, x_axis, y_axis):
        self.model = model
        self.output = mesh  # Expected PyVista mesh type
        self.x_axis = x_axis
        self.y_axis = y_axis
        self.kwargs = {
            'z_axis': len(model),
            'y_axis': len(model[0]),
            'x_axis': len(model[0][0])
        }

    def __call__(self, param, value):
        self.kwargs[param] = value
        if param == "z_axis":
            self.update_z_axis(value)
        elif param == "y_axis":
            self.update_y_axis(value)
        elif param == "x_axis":
            self.update_x_axis(value)

    def value_to_num(self, value, axis):
        def find_elem(value, axis):
            min_bound = 0
            max_bound = len(axis)
            while True:
                if max_bound - min_bound == 1:
                    return max_bound 
                if value == axis[(max_bound + min_bound) // 2]:
                    return (max_bound + min_bound) // 2 + 1
                elif value > axis[(max_bound + min_bound) // 2]:
                    min_bound = (max_bound + min_bound) // 2
                else:
                    max_bound = (max_bound + min_bound) // 2

        match axis:
            case "y_axis":
                return find_elem(value, self.y_axis)
            case "x_axis":
                return find_elem(value, self.x_axis)
        return value


    def update_z_axis(self, value):
        new_model = self.model[:int(value)]
        mesh = pv.ImageData()
        mesh.dimensions = np.array((len(new_model[0][0]),len(new_model[0]), len(new_model))) + 1
        mesh.origin = (0, 0, 0)
        mesh.spacing = (1, 1, 1)
        mesh.cell_data["values"] = new_model.flatten()
        self.output.copy_from(mesh)

    def update_y_axis(self, value):
        new_model = self.model[:, :self.value_to_num(value, "y_axis"), :]
        mesh = pv.ImageData()
        mesh.dimensions = np.array((len(new_model[0][0]),len(new_model[0]), len(new_model))) + 1
        mesh.origin = (0, 0, 0)
        mesh.spacing = (1, 1, 1)
        mesh.cell_data["values"] = new_model.flatten()
        self.output.copy_from(mesh)

    def update_x_axis(self, value):
        new_model = self.model[:, :, :self.value_to_num(value, "x_axis")]
        mesh = pv.ImageData()
        mesh.dimensions = np.array((len(new_model[0][0]),len(new_model[0]), len(new_model))) + 1
        mesh.origin = (0, 0, 0)
        mesh.spacing = (1, 1, 1)
        mesh.cell_data["values"] = new_model.flatten()
        self.output.copy_from(mesh)

def main():
    try:
        model_file = open("../my_model.json")
    except FileNotFoundError:
         model_file = open("./my_model.json")
    # model_file = open("../target/release/my_model.json")
    print("loading", model_file)
    model_file = json.load(model_file)

    model = []
    for i, e in enumerate(model_file["model"]):
        model.append([])
        for j, ee in enumerate(e[f"depth_{i}"]):
            model[-1].append([])
            for k in ee[f"y_{j}"]:
                model[-1][-1].append(int(k))
    x_axis = []
    for _, e in enumerate(model_file["params3D"]["x_ax"]["axis"]):
        x_axis.append(float(e))

    y_axis = []
    for _, e in enumerate(model_file["params3D"]["y_ax"]["axis"]):
        y_axis.append(float(e))

    model = np.array(model[::-1])
    
    print("model size:", model.shape)

    mesh = pv.ImageData()
    mesh.dimensions = np.array((len(model[0][0]),len(model[0]), len(model))) + 1
    mesh.origin = (0, 0, 0)
    mesh.spacing = (1, 1, 1)
    mesh.cell_data["values"] = model.flatten()

    p = pv.Plotter()
    p.add_mesh(mesh, opacity=1, show_edges=True)
    p.show_bounds(axes_ranges=[x_axis[0], x_axis[-1], y_axis[0], y_axis[-1], 1.0, 7.0])

    engine = ViewerEngine(mesh, model.copy(), x_axis, y_axis)

    p.add_slider_widget(
        callback=lambda value: engine('z_axis', value),
        rng=[1, len(model)],
        value=len(model),
        title="Z axis",
        pointa=(0.025, 0.1),
        pointb=(0.31, 0.1),
        style="modern"
    )
    p.add_slider_widget(
        callback=lambda value: engine('y_axis', value),
        rng=[y_axis[0], y_axis[-1]],
        value=y_axis[-1],
        title="Y axis",
        pointa=(0.025, 0.2),
        pointb=(0.31, 0.2),
        style="modern"
    )
    p.add_slider_widget(
        callback=lambda value: engine('x_axis', value),
        rng=[x_axis[0], x_axis[-1]],
        value=x_axis[-1],
        title="X axis",
        pointa=(0.025, 0.3),
        pointb=(0.31, 0.3),
        style="modern"
    )

    p.show()

if __name__ == "__main__":
    main()

