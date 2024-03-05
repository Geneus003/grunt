import numpy as np
import json
import pyvista as pv

class ViewerEngine:
    def __init__(self, mesh, model):
        self.model = model
        self.output = mesh  # Expected PyVista mesh type
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

    def update_z_axis(self, value):
        new_model = self.model[:value]
        mesh = pv.ImageData()
        mesh.dimensions = np.array((len(new_model[0][0]),len(new_model[0]), len(new_model))) + 1
        mesh.origin = (0, 0, 0)
        mesh.spacing = (1, 1, 1)
        mesh.cell_data["values"] = new_model.flatten()
        self.output.copy_from(mesh)

    def update_y_axis(self, value):
        new_model = self.model[:, :value, :]
        mesh = pv.ImageData()
        mesh.dimensions = np.array((len(new_model[0][0]),len(new_model[0]), len(new_model))) + 1
        mesh.origin = (0, 0, 0)
        mesh.spacing = (1, 1, 1)
        mesh.cell_data["values"] = new_model.flatten()
        self.output.copy_from(mesh)

    def update_x_axis(self, value):
        new_model = self.model[:, :, :value]
        mesh = pv.ImageData()
        mesh.dimensions = np.array((len(new_model[0][0]),len(new_model[0]), len(new_model))) + 1
        mesh.origin = (0, 0, 0)
        mesh.spacing = (1, 1, 1)
        mesh.cell_data["values"] = new_model.flatten()
        self.output.copy_from(mesh)

def main():
    model_file = open("../my_model.json")
    # model_file = open("../target/release/my_model.json")
    model_file = json.load(model_file)

    model = []
    for i, e in enumerate(model_file["model"]):
        model.append([])
        for j, ee in enumerate(e[f"depth_{i}"]):
            model[-1].append([])
            for k in ee[f"y_{j}"]:
                model[-1][-1].append(int(k))

    model = np.array(model[::-1])

    mesh = pv.ImageData()
    mesh.dimensions = np.array((len(model[0][0]),len(model[0]), len(model))) + 1
    mesh.origin = (0, 0, 0)
    mesh.spacing = (1, 1, 1)

    mesh.cell_data["values"] = model.flatten()

    p = pv.Plotter()
    p.add_mesh(mesh, opacity=1, show_edges=True)
    p.show_bounds(mesh=mesh)

    engine = ViewerEngine(mesh, model.copy())

    p.add_slider_widget(
        callback=lambda value: engine('z_axis', int(value)),
        rng=[1, len(model)],
        value=len(model),
        title="Z axis",
        pointa=(0.025, 0.1),
        pointb=(0.31, 0.1),
        style="modern"
    )
    p.add_slider_widget(
        callback=lambda value: engine('y_axis', int(value)),
        rng=[1, len(model[0])],
        value=len(model[0]),
        title="Y axis",
        pointa=(0.025, 0.2),
        pointb=(0.31, 0.2),
        style="modern"
    )
    p.add_slider_widget(
        callback=lambda value: engine('x_axis', int(value)),
        rng=[1, len(model[0][0])],
        value=len(model[0][0]),
        title="X axis",
        pointa=(0.025, 0.3),
        pointb=(0.31, 0.3),
        style="modern"
    )
    p.show()


if __name__ == "__main__":
    main()

