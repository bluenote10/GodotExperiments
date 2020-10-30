import uvicorn
from fastapi import FastAPI

import numpy as np

app = FastAPI()


@app.get("/")
def root():

    data = np.random.uniform(size=(100, 200))

    return {"data": data.tolist()}


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
