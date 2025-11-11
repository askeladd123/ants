import { Environment } from '../wasm/ants.js';
import * as THREE from 'three';

if (typeof ESBUILD_LIVE_RELOAD !== 'undefined' && ESBUILD_LIVE_RELOAD) {
  new EventSource("/esbuild").addEventListener("change", () => location.reload());
}

const FRAME_RATIO = 2 / 3;
const width = function() { return window.innerWidth };
const height = function() { return window.innerWidth * FRAME_RATIO };

const renderer = new THREE.WebGLRenderer({ antialias: true });
const scene = new THREE.Scene();
const camera = new THREE.OrthographicCamera(width() / - 2, width() / 2, height() / 2, height() / - 2, -100, 100);
scene.add(camera);
renderer.setSize(width(), height());
document.body.appendChild(renderer.domElement);

const instances = 1000;
const grid_uni_sparse_nx = 8;
const grid_uni_sparse_ny = Math.round(grid_uni_sparse_nx * FRAME_RATIO);
// const grid_uniform_nx = 1200;
const grid_uniform_nx = 16;
const grid_uniform_ny = Math.round(grid_uniform_nx * FRAME_RATIO);
const grid_uniform_n = grid_uniform_nx * grid_uniform_ny;
const grid_uni_sparse_n = grid_uni_sparse_nx * grid_uni_sparse_ny;
const environment = new Environment(instances, width(), height(), grid_uni_sparse_nx, grid_uni_sparse_ny);

const vertices = [
  0, 6, 0, // top
  4, 2, 0, // right
  0, -10, 0, // bottom
  -4, 2, 0, // left
];
const scale = 1.2;

const indices = [
  0, 3, 1, // top-right-bottom
  1, 3, 2, // top-bottom-left
];

const geometry = new THREE.BufferGeometry();
geometry.setAttribute(
  'position',
  new THREE.BufferAttribute(new Float32Array(vertices), 3)
);
geometry.setIndex(indices);
geometry.scale(scale, scale, 1);

const material = new THREE.MeshBasicMaterial({ color: new THREE.Color('skyblue') });


const mesh_uni_sparse = new THREE.InstancedMesh(geometry, material, instances);

function init_uniform(width, height) {
  const material = new THREE.MeshBasicMaterial({ color: new THREE.Color('white') }); // WARN: don't know why this has to be white initially
  const geom = new THREE.PlaneGeometry(1, 1);
  const instanced = new THREE.InstancedMesh(geom, material, grid_uniform_n);
  const cellW = width / grid_uniform_nx;
  const cellH = height / grid_uniform_ny;
  const tmp = new THREE.Matrix4();

  let i = 0;
  for (let y = 0; y < grid_uniform_ny; y++) {
    for (let x = 0; x < grid_uniform_nx; x++) {
      const cx = -width / 2 + (x + 0.5) * cellW;
      const cy = height / 2 - (y + 0.5) * cellH;
      tmp.identity()
        .makeScale(cellW, cellH, 0)
        .setPosition(cx, cy, -1);
      instanced.setMatrixAt(i++, tmp);
    }
  }
  instanced.instanceMatrix.needsUpdate = true;
  return instanced;
}

const mesh_uniform = init_uniform(width(), height());

scene.add(mesh_uni_sparse);
scene.add(mesh_uniform);

camera.position.z = 5;

const mouseLabel = document.getElementById('mouse-coord-debug');

const raycaster = new THREE.Raycaster();
const mouseNDC = new THREE.Vector2();
const worldPos = new THREE.Vector3();
const planeZ = new THREE.Plane(new THREE.Vector3(0, 0, 1), 0);

renderer.domElement.addEventListener('mousemove', (event) => {
  const rect = renderer.domElement.getBoundingClientRect();

  mouseNDC.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
  mouseNDC.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

  raycaster.setFromCamera(mouseNDC, camera);
  raycaster.ray.intersectPlane(planeZ, worldPos);

  mouseLabel.textContent = `x: ${worldPos.x.toFixed(1)}  y: ${worldPos.y.toFixed(1)}`;
});

const debug_grid_planes = [];

for (let i = 0; i < grid_uni_sparse_n; i++) {
  let tmp = [];
  for (let j = 0; j < 2; j++) {
    const debug_grid_plane_geometry = new THREE.PlaneGeometry(1, 1);
    const debug_grid_plane_material = new THREE.MeshBasicMaterial({ color: new THREE.Color('grey') });
    const debug_grid_plane_mesh = new THREE.Mesh(debug_grid_plane_geometry, debug_grid_plane_material);
    debug_grid_plane_mesh.visible = false;
    scene.add(debug_grid_plane_mesh);
    tmp.push(debug_grid_plane_mesh);
  }
  debug_grid_planes.push(tmp);
}

let debug = sessionStorage.getItem("settings");
if (debug == null) {
  debug = false;
} else {
  debug = JSON.parse(debug).debug;
}
debug_init();

const clock = new THREE.Clock();

renderer.setAnimationLoop(() => {

  const last_environment = environment.step(clock.getDelta(), debug);

  let i = 0
  for (const bucket of last_environment.main.buckets) {
    for (const ant of bucket.ants) {

      let obj = new THREE.Object3D();
      obj.position.x = ant.x;
      obj.position.y = ant.y;
      obj.rotation.z = ant.a - Math.PI / 2;
      obj.updateMatrix();

      let color = null;
      if (debug) {
        color = new THREE.Color().setHSL((6.789 * bucket.index / grid_uni_sparse_n) % 1, 0.4, 0.5);
      } else {
        color = new THREE.Color('skyblue');
      }

      mesh_uni_sparse.setMatrixAt(i, obj.matrix);
      mesh_uni_sparse.setColorAt(i, color);
      i++;
    }
  }

  if (debug) {
    const grid = last_environment.debug.grid_uni_sparse;
    for (let i = 0; i < grid_uni_sparse_n; i++) {
      let bucket = grid.buckets[i];
      let pair = debug_grid_planes[i];

      const thickness = 0.5;
      const rx = grid.bucket_width / 2;
      const ry = grid.bucket_height / 2;
      pair[0].position.set(bucket.x + rx, bucket.y, 0);
      pair[0].scale.set(grid.bucket_width, thickness, 1);
      pair[1].position.set(bucket.x, bucket.y + ry, 0);
      pair[1].scale.set(thickness, grid.bucket_height, 1);
    }

    const grid_uniform = last_environment.debug.grid_uniform.entries;
    for (let i = 0; i < grid_uniform_n; i++) {
      if (grid_uniform[i] == "Empty") {
        mesh_uniform.setColorAt(i, new THREE.Color('black'));
      }
      try {
        if ("Food" in grid_uniform[i]) {
          mesh_uniform.setColorAt(i, new THREE.Color('green'));

        }
      } catch { }
    }
    mesh_uniform.instanceColor.needsUpdate = true;
  }

  mesh_uni_sparse.instanceMatrix.needsUpdate = true;
  mesh_uni_sparse.instanceColor.needsUpdate = true;

  renderer.render(scene, camera);
});

window.addEventListener('resize', () => {
  const w = width();
  const h = height();

  camera.left = w / -2;
  camera.right = w / 2;
  camera.top = h / 2;
  camera.bottom = h / -2;
  camera.updateProjectionMatrix();

  renderer.setSize(w, h);
});

function debug_init() {
  for (const pair of debug_grid_planes) {
    for (const m of pair) {
      m.visible = true;
    }
  }
}

function debug_clear() {
  for (const pair of debug_grid_planes) {
    for (const m of pair) {
      m.visible = false;
    }
  }
}

let button = document.getElementById('button-debug');
button.addEventListener('click', () => {
  if (debug) {
    debug = false;
    button.textContent = 'debug: enable';
    debug_clear();
    document.getElementById("debug").style.display = 'none';

  } else {
    debug = true;
    button.textContent = 'debug: disable';
    debug_init();
    document.getElementById("debug").style.display = 'block';
  }
  sessionStorage.setItem("settings", JSON.stringify({ debug: debug }));
})
