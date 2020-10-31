import os
import multiprocessing
import subprocess
import time

import uvicorn
from fastapi import FastAPI

import numpy as np


def create_app(data):
    app = FastAPI()

    data_response = {"data": data.tolist()}

    @app.get("/")
    def root():
        return data_response

    return app


def run_app(data, scale=10.0):
    # normalize data
    data_min = data.min()
    data_max = data.max()
    data = (data - data_min) / (data_max - data_min)

    app = create_app(data)
    uvicorn.run(app, host="0.0.0.0", port=8000)


def run_godot(godot_path):
    godot_project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    process = subprocess.Popen(godot_path, cwd=godot_project_root)
    process.wait()


class Goplot(object):
    def __init__(self, godot_path):
        self.godot_path = godot_path

    def plot_surface(self, data, keep_backend_running=False):
        process_backend = multiprocessing.Process(target=run_app, args=(data,))
        process_backend.start()

        # TODO: Could be replaced by health status poll
        time.sleep(0.1)

        process_godot = multiprocessing.Process(target=run_godot, args=(self.godot_path,))
        process_godot.start()

        print("Waiting for Godot to terminate...")
        process_godot.join()

        if not keep_backend_running:
            process_backend.terminate()


if __name__ == "__main__":
    if False:
        app = FastAPI()

        @app.get("/")
        def root():
            data = np.random.uniform(size=(100, 200))
            return {"data": data.tolist()}

        uvicorn.run(app, host="0.0.0.0", port=8000)

    if True:
        data = np.outer(np.sin(np.arange(200) / 10), np.arange(10)) ** 2
        goplot = Goplot("/home/fabian/bin/Godot_v3.2.2-stable_x11.64")
        goplot.plot_surface(data, keep_backend_running=True)
