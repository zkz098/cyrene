import type { TreeNode } from '../utils/buildFileTree'
import type { postFile } from '../utils/types'
import { defineStore } from 'pinia'

export const useFilesStore = defineStore('files', {
  state: () => ({
    basePath: '',
    files: {} as Record<string, postFile>,
    filetree: [] as TreeNode[],
    ready: {
      selectedFile: false,
      fileList: false,
      fileTree: false,
      fileContent: false,
      xlsxExporting: false,
    },
  }),

  actions: {
    getFileAbsolutePathList() {
      return Object.keys(this.files)
    },
    getFileRelativePathList() {
      return Object.values(this.files).map(file => file.relativePath)
    },
  },
})
