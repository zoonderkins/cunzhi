<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  alwaysOnTop: {
    type: Boolean,
    required: true,
  },
  windowWidth: {
    type: Number,
    default: 600,
  },
  windowHeight: {
    type: Number,
    default: 900,
  },
})

const emit = defineEmits(['toggleAlwaysOnTop', 'updateWindowSize'])

// 窗口尺寸设置
const localWidth = ref(props.windowWidth)
const localHeight = ref(props.windowHeight)
const fixedWindowSize = ref(false)

// 最小尺寸限制
const minWidth = 500
const minHeight = 500

// 步长设置
const step = 50

// 保存窗口尺寸
function saveWindowSize() {
  // 确保不小于最小值
  if (localWidth.value < minWidth)
    localWidth.value = minWidth
  if (localHeight.value < minHeight)
    localHeight.value = minHeight

  emit('updateWindowSize', {
    width: localWidth.value,
    height: localHeight.value,
    fixed: fixedWindowSize.value,
  })
}

// 监听固定窗口大小变化
watch(fixedWindowSize, () => {
  saveWindowSize()
})
</script>

<template>
  <n-card size="small">
    <!-- 卡片头部 -->
    <template #header>
      <n-space align="center">
        <!-- 图标 -->
        <div class="w-10 h-10 rounded-lg bg-green-100 dark:bg-green-900 flex items-center justify-center">
          <div class="i-carbon-settings text-lg text-gray-700 dark:text-gray-200" />
        </div>

        <!-- 标题和副标题 -->
        <div>
          <div class="text-lg font-medium mb-1 tracking-tight">
            窗口设置
          </div>
          <div class="text-sm opacity-60 font-normal">
            配置窗口显示行为
          </div>
        </div>
      </n-space>
    </template>

    <!-- 设置内容 -->
    <n-space vertical size="large">
      <!-- 置顶显示设置 -->
      <div class="flex items-center justify-between">
        <div class="flex items-center">
          <div class="w-1.5 h-1.5 bg-green-500 rounded-full mr-3 flex-shrink-0" />
          <div>
            <div class="text-sm font-medium leading-relaxed">
              总在最前
            </div>
            <div class="text-xs opacity-60">
              启用后窗口将始终保持在其他应用程序之上
            </div>
          </div>
        </div>
        <n-switch
          :value="alwaysOnTop"
          size="small"
          @update:value="$emit('toggleAlwaysOnTop')"
        />
      </div>

      <!-- 窗口尺寸设置 -->
      <div class="pt-4 border-t border-gray-200 dark:border-gray-700">
        <div class="flex items-start">
          <div class="w-1.5 h-1.5 bg-green-500 rounded-full mr-3 mt-2 flex-shrink-0" />
          <div class="flex-1">
            <div class="text-sm font-medium mb-3 leading-relaxed">
              窗口尺寸
            </div>

            <!-- 窗口模式选择 -->
            <div class="space-y-3">
              <!-- 自由拉伸模式 -->
              <div
                class="p-3 rounded-lg border cursor-pointer transition-all"
                :class="!fixedWindowSize ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20 shadow-sm' : 'border-gray-300 dark:border-gray-600 hover:border-blue-300 dark:hover:border-blue-500 hover:bg-gray-50 dark:hover:bg-gray-800'"
                @click="fixedWindowSize = false"
              >
                <div class="flex items-center justify-between">
                  <div class="flex items-center">
                    <div
                      class="w-4 h-4 rounded-full border-2 mr-3 flex items-center justify-center"
                      :class="!fixedWindowSize ? 'border-blue-500' : 'border-gray-400 dark:border-gray-500'"
                    >
                      <div
                        v-if="!fixedWindowSize"
                        class="w-2 h-2 bg-blue-500 rounded-full"
                      />
                    </div>
                    <div>
                      <div class="text-sm font-medium mb-1">
                        自由拉伸
                      </div>
                      <div class="text-xs opacity-60">
                        窗口可以自由拖拽调整大小
                      </div>
                    </div>
                  </div>
                  <div class="text-xs opacity-60">
                    默认 {{ localWidth }} × {{ localHeight }} px
                  </div>
                </div>
              </div>

              <!-- 固定大小模式 -->
              <div
                class="p-3 rounded-lg border cursor-pointer transition-all"
                :class="fixedWindowSize ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20 shadow-sm' : 'border-gray-300 dark:border-gray-600 hover:border-blue-300 dark:hover:border-blue-500 hover:bg-gray-50 dark:hover:bg-gray-800'"
                @click="fixedWindowSize = true"
              >
                <div class="flex items-center justify-between">
                  <div class="flex items-center">
                    <div
                      class="w-4 h-4 rounded-full border-2 mr-3 flex items-center justify-center"
                      :class="fixedWindowSize ? 'border-blue-500' : 'border-gray-400 dark:border-gray-500'"
                    >
                      <div
                        v-if="fixedWindowSize"
                        class="w-2 h-2 bg-blue-500 rounded-full"
                      />
                    </div>
                    <div>
                      <div class="text-sm font-medium mb-1">
                        固定大小
                      </div>
                      <div class="text-xs opacity-60">
                        设置固定的窗口尺寸
                      </div>
                    </div>
                  </div>
                  <div class="text-xs opacity-60">
                    {{ localWidth }} × {{ localHeight }} px
                  </div>
                </div>

                <!-- 固定模式的尺寸设置 -->
                <div v-if="fixedWindowSize" class="mt-3 pt-3 border-t border-gray-200 dark:border-gray-600">
                  <div class="grid grid-cols-2 gap-3">
                    <!-- 宽度设置 -->
                    <div>
                      <div class="text-xs opacity-60 mb-2">
                        宽度 (最小500px)
                      </div>
                      <n-input-number
                        v-model:value="localWidth"
                        :min="minWidth"
                        :max="2000"
                        :step="step"
                        size="small"
                        placeholder="宽度"
                      />
                    </div>

                    <!-- 高度设置 -->
                    <div>
                      <div class="text-xs opacity-60 mb-2">
                        高度 (最小500px)
                      </div>
                      <n-input-number
                        v-model:value="localHeight"
                        :min="minHeight"
                        :max="1500"
                        :step="step"
                        size="small"
                        placeholder="高度"
                      />
                    </div>
                  </div>

                  <!-- 保存按钮 -->
                  <div class="mt-3 pt-3 border-t border-gray-200 dark:border-gray-600">
                    <n-button
                      type="primary"
                      size="small"
                      @click="saveWindowSize"
                    >
                      保存窗口设置
                    </n-button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </n-space>
  </n-card>
</template>
