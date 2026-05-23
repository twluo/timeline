<script setup lang="ts">
import { ref, onMounted } from "vue";
import { setOptions, importLibrary } from "@googlemaps/js-api-loader";

const props = defineProps<{
  points: {
    lat: number;
    lng: number;
    timestamp: string;
    name: string;
    address: string;
  }[];
}>();

const mapEl = ref<HTMLDivElement | null>(null);
const currentIndex = ref(0);

type AnyMarker = { addListener: (event: string, handler: () => void) => void };

// Set after async map setup so the nav buttons can trigger them
let markers: AnyMarker[] = [];
let navigateTo: (index: number) => void = () => {};

function prev() {
  if (currentIndex.value > 0) navigateTo(currentIndex.value - 1);
}

function next() {
  if (currentIndex.value < props.points.length - 1) navigateTo(currentIndex.value + 1);
}

// Green for first, red for last, blue for everything in between
function pinColor(index: number, total: number) {
  if (index === 0) return { background: "#1a9e3f", borderColor: "#0f6e2b", glyphColor: "#fff" };
  if (index === total - 1)
    return { background: "#d93025", borderColor: "#9e1f16", glyphColor: "#fff" };
  return { background: "#4285F4", borderColor: "#1a56c4", glyphColor: "#fff" };
}

function buildInfoContent(point: (typeof props.points)[number], onClose: () => void) {
  const wrap = document.createElement("div");
  Object.assign(wrap.style, {
    fontFamily: "sans-serif",
    minWidth: "220px",
    padding: "4px 2px",
  });

  const header = document.createElement("div");
  Object.assign(header.style, {
    display: "flex",
    justifyContent: "space-between",
    alignItems: "flex-start",
    marginBottom: "8px",
  });

  const name = document.createElement("div");
  name.textContent = point.name;
  Object.assign(name.style, {
    fontSize: "15px",
    fontWeight: "600",
    color: "#1a1a1a",
  });

  const closeBtn = document.createElement("button");
  closeBtn.textContent = "\u00D7";
  Object.assign(closeBtn.style, {
    background: "none",
    border: "none",
    cursor: "pointer",
    fontSize: "18px",
    lineHeight: "1",
    color: "#888",
    padding: "0 0 0 8px",
    flexShrink: "0",
  });
  closeBtn.addEventListener("click", onClose);

  header.append(name, closeBtn);

  const divider = document.createElement("hr");
  Object.assign(divider.style, {
    border: "none",
    borderTop: "1px solid #e0e0e0",
    margin: "0 0 8px",
  });

  const address = document.createElement("div");
  address.textContent = `\uD83D\uDCCD\u2009${point.address}`;
  Object.assign(address.style, {
    fontSize: "13px",
    color: "#555",
    marginBottom: "6px",
    lineHeight: "1.4",
  });

  const time = document.createElement("div");
  time.textContent = `\uD83D\uDD52\u2009${new Date(point.timestamp).toLocaleString()}`;
  Object.assign(time.style, {
    fontSize: "12px",
    color: "#888",
  });

  wrap.append(header, divider, address, time);
  return wrap;
}

onMounted(async () => {
  setOptions({
    key: import.meta.env.VITE_GOOGLE_MAPS_API_KEY,
    v: "weekly",
  });

  const mapId = "DEMO_MAP_ID";

  const { Map, InfoWindow } = await importLibrary("maps");
  const { LatLngBounds } = await importLibrary("core");

  if (!mapEl.value || props.points.length === 0) return;

  const map = new Map(mapEl.value, {
    center: { lat: 0, lng: 0 },
    zoom: 2,
    ...(mapId ? { mapId } : {}),
  });

  const bounds = new LatLngBounds();
  for (const point of props.points) bounds.extend(point);
  map.fitBounds(bounds);

  const infoWindow = new InfoWindow({ headerDisabled: true });

  const { AdvancedMarkerElement, PinElement } = mapId
    ? await importLibrary("marker")
    : { AdvancedMarkerElement: null, PinElement: null };
  const { Marker } = mapId ? { Marker: null } : await importLibrary("maps");

  markers = props.points.map((point, index) => {
    const label = String(index + 1);
    const colors = pinColor(index, props.points.length);

    const marker =
      AdvancedMarkerElement && PinElement
        ? new AdvancedMarkerElement({
            map,
            position: point,
            content: new PinElement({ glyph: label, ...colors }).element,
          })
        : new Marker!({ map, position: point, label });

    marker.addListener("click", () => {
      currentIndex.value = index;
      infoWindow.setContent(buildInfoContent(point, () => infoWindow.close()));
      infoWindow.open({ anchor: marker, map });
    });

    return marker;
  });

  navigateTo = (index: number) => {
    const point = props.points[index];
    const marker = markers[index];
    if (!point || !marker) return;

    currentIndex.value = index;
    map.panTo(point);
    infoWindow.setContent(buildInfoContent(point, () => infoWindow.close()));
    infoWindow.open({ anchor: marker, map });
  };
});
</script>

<template>
  <div class="wrapper">
    <div ref="mapEl" class="map" />
    <div class="controls">
      <button :disabled="currentIndex === 0" @click="prev">&#8592;</button>
      <span>{{ currentIndex + 1 }} / {{ points.length }}</span>
      <button :disabled="currentIndex === points.length - 1" @click="next">&#8594;</button>
    </div>
  </div>
</template>

<style scoped>
.wrapper {
  position: relative;
  width: 100%;
  height: 100%;
}

.map {
  width: 100%;
  height: 100%;
}

.controls {
  position: absolute;
  bottom: 2rem;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 1rem;
  background: white;
  border-radius: 999px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  font-size: 0.9rem;
  user-select: none;
}

.controls button {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 1.1rem;
  padding: 0 0.25rem;
  line-height: 1;
}

.controls button:disabled {
  opacity: 0.3;
  cursor: default;
}
</style>
