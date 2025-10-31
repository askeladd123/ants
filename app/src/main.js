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

const instances = 100;
const environment = new Environment(instances, width(), height());

const verticies = [
  [-4, 2, 0],
  [0, -10, 0],
  [4, 2, 0],
];
const geometry = new THREE.BufferGeometry(); // TODO: create complex, indexed geometry
geometry.setAttribute(
  'position',
  new THREE.BufferAttribute(new Float32Array(verticies.flat()), 3)
);

const material = new THREE.MeshBasicMaterial({ color: new THREE.Color('skyblue') });
const mesh = new THREE.InstancedMesh(geometry, material, instances);
scene.add(mesh);

camera.position.z = 5;

renderer.setAnimationLoop(() => {
  let last_environment = environment.step();
  for (let i = 0; i < instances; i++) {
    let ant = last_environment.ants[i];
    let obj = new THREE.Object3D();
    obj.position.x = ant.x;
    obj.position.y = ant.y;
    obj.rotation.z = ant.a - Math.PI / 2;
    obj.updateMatrix();
    mesh.setMatrixAt(i, obj.matrix);
  }
  mesh.instanceMatrix.needsUpdate = true;
  renderer.render(scene, camera);
});
window.addEventListener('resize', () => {
  camera.aspect = width() / height();
  camera.updateProjectionMatrix();
  renderer.setSize(width(), height());
});
