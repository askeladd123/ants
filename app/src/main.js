// import { } from '../wasm/ants.js';
import * as THREE from 'three';
import { BufferGeometryUtils } from 'three/examples/jsm/Addons.js';

if (typeof ESBUILD_LIVE_RELOAD !== 'undefined' && ESBUILD_LIVE_RELOAD) {
  new EventSource("/esbuild").addEventListener("change", () => location.reload());
}

const width = function() { return window.innerWidth };
const height = function() { return window.innerWidth * 2 / 3 };
const renderer = new THREE.WebGLRenderer();
const scene = new THREE.Scene();
const camera = new THREE.OrthographicCamera(width() / - 2, width() / 2, height() / 2, height() / - 2, -100, 100);
scene.add(camera);
renderer.setSize(width(), height());
document.body.appendChild(renderer.domElement);

const instances = 1000
const material = new THREE.MeshBasicMaterial({ color: new THREE.Color('skyblue') });
const geometry = new THREE.TetrahedronGeometry(10, 1);
const mesh = new THREE.InstancedMesh(geometry, material, instances)

const SPREAD = 20;
const cols = 50;
const rows = instances / cols;
const grid_width = cols * SPREAD;
const grid_height = rows * SPREAD;
for (let y = 0; y < rows; y++) {
  for (let x = 0; x < cols; x++) {
    let obj = new THREE.Object3D();
    obj.position.x = x * SPREAD - grid_width / 2;
    obj.position.y = y * SPREAD - grid_height / 2;
    obj.updateMatrix();
    mesh.setMatrixAt(y * cols + x, obj.matrix)
    mesh.setColorAt(
      y * cols + x,
      new THREE.Color().setRGB(
        obj.position.x / grid_width,
        obj.position.y / grid_height,
        obj.position.x / grid_width * 2
      )
    );
  }
}

// const material = new THREE.MeshBasicMaterial({ color: new THREE.Color('skyblue') });
// const geometry = new THREE.BoxGeometry(10, 10, 0);
// const mesh = new THREE.Mesh(geometry, material);
// const geometries = [mesh];
// const mergedGeometry = BufferGeometryUtils.mergeGeometries(geometries, false)

// scene.add(mergedGeometry)
scene.add(mesh);

camera.position.z = 5;

renderer.setAnimationLoop(() => {
  renderer.render(scene, camera);
});
window.addEventListener('resize', () => {
  camera.aspect = width() / height();
  camera.updateProjectionMatrix();
  renderer.setSize(width(), height());
});
