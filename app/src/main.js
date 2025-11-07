import { Environment } from '../wasm/ants.js';
import * as THREE from 'three';

if (typeof ESBUILD_LIVE_RELOAD !== 'undefined' && ESBUILD_LIVE_RELOAD) {
  new EventSource("/esbuild").addEventListener("change", () => location.reload());
}

const width = function() { return window.innerWidth };
const height = function() { return window.innerWidth * 2 / 3 };

const renderer = new THREE.WebGLRenderer({ antialias: true });
const scene = new THREE.Scene();
const camera = new THREE.OrthographicCamera(width() / - 2, width() / 2, height() / 2, height() / - 2, -100, 100);
scene.add(camera);
renderer.setSize(width(), height());
document.body.appendChild(renderer.domElement);

const instances = 1000;
const grid_nx = 8;
const grid_ny = 5;
const grid_n = grid_nx * grid_ny;
const environment = new Environment(instances, width(), height(), grid_nx, grid_ny);

const vertices = [
  0, 6, 0, // top
  4, 2, 0, // right
  0, -10, 0, // bottom
  -4, 2, 0, // left
];

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

const material = new THREE.MeshBasicMaterial({ color: new THREE.Color('skyblue') });
const mesh = new THREE.InstancedMesh(geometry, material, instances);
const scale = 1.5;
mesh.scale.set(scale, scale, 1);

scene.add(mesh);

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
// const buckets_meta = environment.get_info().buckets_meta;

for (let i = 0; i < grid_n; i++) {
  let tmp = [];
  for (let j = 0; j < 2; j++) {
    const debug_grid_plane_geometry = new THREE.PlaneGeometry(1, 1);
    const debug_grid_plane_material = new THREE.MeshBasicMaterial({ color: 0x2196f3 });
    const debug_grid_plane_mesh = new THREE.Mesh(debug_grid_plane_geometry, debug_grid_plane_material);
    debug_grid_plane_mesh.visible = false;
    scene.add(debug_grid_plane_mesh);
    tmp.push(debug_grid_plane_mesh);
  }
  debug_grid_planes.push(tmp);
}

let debug = true;
debug_init();

renderer.setAnimationLoop(() => {
  const last_environment = environment.step(debug);

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
        color = new THREE.Color().setHSL((1.137 * bucket.index / grid_n) % 1, 0.8, 0.5);
      } else {
        color = new THREE.Color('skyblue');
      }

      mesh.setMatrixAt(i, obj.matrix);
      mesh.setColorAt(i, color);
      i++;
    }
  }

  // if (debug) {
  //   for (let i = 0; i < grid_n; i++) {
  //     let bucket = buckets_meta[i];
  //     let pair = debug_grid_planes[i];

  //     pair[0].position.set(bucket.x, bucket.y, 0);
  //     pair[0].scale.set(bucket.w, 1, 0);
  //     pair[1].position.set(bucket.x, bucket.y, 0);
  //     pair[1].scale.set(1, bucket.w, 0);
  //   }
  // }

  mesh.instanceMatrix.needsUpdate = true;
  mesh.instanceColor.needsUpdate = true;

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
})
