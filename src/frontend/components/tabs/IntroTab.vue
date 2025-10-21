<script setup lang="ts">
import { computed } from 'vue'
import FeatureCard from '../common/FeatureCard.vue'
import ProjectInfoCard from '../common/ProjectInfoCard.vue'
import { useI18n } from '../../i18n'

const { t } = useI18n()

const features = computed(() => [
  {
    icon: 'i-carbon-chat text-lg text-blue-600 dark:text-blue-400',
    title: t('intro.features.zhiTitle'),
    subtitle: t('intro.features.zhiSubtitle'),
    iconBg: 'bg-blue-100',
    features: [
      t('intro.features.zhiFeature1'),
      t('intro.features.zhiFeature2'),
      t('intro.features.zhiFeature3'),
      t('intro.features.zhiFeature4'),
      t('intro.features.zhiFeature5'),
    ],
  },
  {
    icon: 'i-carbon-data-base text-lg text-purple-600 dark:text-purple-400',
    title: t('intro.features.jiTitle'),
    subtitle: t('intro.features.jiSubtitle'),
    iconBg: 'bg-purple-100',
    features: [
      t('intro.features.jiFeature1'),
      t('intro.features.jiFeature2'),
      t('intro.features.jiFeature3'),
      t('intro.features.jiFeature4'),
      t('intro.features.jiFeature5'),
    ],
  },
  {
    icon: 'i-carbon-volume-up text-lg text-orange-600 dark:text-orange-400',
    title: t('intro.features.audioTitle'),
    subtitle: t('intro.features.audioSubtitle'),
    iconBg: 'bg-orange-100',
    features: [
      t('intro.features.audioFeature1'),
      t('intro.features.audioFeature2'),
      t('intro.features.audioFeature3'),
      t('intro.features.audioFeature4'),
      t('intro.features.audioFeature5'),
    ],
  },
  {
    icon: 'i-carbon-settings text-lg text-green-600 dark:text-green-400',
    title: t('intro.features.settingsTitle'),
    subtitle: t('intro.features.settingsSubtitle'),
    iconBg: 'bg-green-100',
    features: [
      t('intro.features.settingsFeature1'),
      t('intro.features.settingsFeature2'),
      t('intro.features.settingsFeature3'),
      t('intro.features.settingsFeature4'),
      t('intro.features.settingsFeature5'),
    ],
  },
])
</script>

<template>
  <div class="tab-content">
    <!-- 项目信息卡片 -->
    <div class="mb-5">
      <ProjectInfoCard />
    </div>

    <!-- 功能卡片 -->
    <n-grid :cols="2" :x-gap="20" :y-gap="20" responsive="screen">
      <n-grid-item
        v-for="(feature, index) in features.value"
        :key="index"
        :span="1"
      >
        <FeatureCard :feature="feature" />
      </n-grid-item>
    </n-grid>
  </div>
</template>
