<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { timelineClient } from "@/api/timelineClient";
import MapView from "@/components/MapView.vue";

interface TimelinePoint {
  user_id: number;
  device_id: string;
  place: {
    id: string;
    name: string;
    coordinate: { latitude: number; longitude: number };
    address: string;
  };
  coordinate: { latitude: number; longitude: number };
  timestamp: string;
}

const response = ref<TimelinePoint[] | null>(null);
const error = ref<string | null>(null);

const points = computed(() =>
  [...(response.value ?? [])]
    .sort((a, b) => new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime())
    .map(({ coordinate, timestamp, place }) => ({
      lat: coordinate.latitude,
      lng: coordinate.longitude,
      timestamp,
      name: place.name,
      address: place.address,
    })),
);

const query = new URLSearchParams(window.location.search);
const userId = query.get("userId");
const deviceId = query.get("deviceId");
const date = query.get("date");

onMounted(async () => {
  if (!userId || !deviceId || !date) {
    error.value = "Missing required query params: userId, deviceId, date";
    return;
  }

  try {
    const params = new URLSearchParams({ userId, deviceId, date });
    response.value = await timelineClient.get<TimelinePoint[]>(`/api/timeline?${params}`);
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  }
});
</script>

<template>
  <div class="layout">
    <pre v-if="error" class="error">{{ error }}</pre>
    <template v-else>
      <MapView v-if="points.length" :points="points" />
      <p v-else class="loading">Loading…</p>
    </template>
  </div>
</template>

<style scoped>
.layout {
  width: 100vw;
  height: 100vh;
}

.error,
.loading {
  padding: 1rem;
}
</style>
