import type { TreeNode } from '../utils/buildFileTree'
import type { postFile } from '../utils/types'
import { defineStore } from 'pinia'

export const useFilesStore = defineStore('files', {
  state: () => ({
    basePath: '',
    currentAccessPath: '',
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
    setCurrentAccessPath(relativePath: string) {
      this.currentAccessPath = this.basePath + relativePath
    },
    addKeyValueToFrontmatter(keys: string[], value: unknown, regexp: RegExp) {
      let modifiedCnt = 0
      Object.values(this.files).forEach((file) => {
        if (regexp.test(file.relativePath)) {
          keys.forEach((key) => {
            if (!(key in file.frontmatter)) {
              file.frontmatter[key] = value
              file.modified = true
              modifiedCnt++
            }
          })
        }
      })

      return modifiedCnt
    },
    removeKeyFromFrontmatter(keys: string[], regexp: RegExp) {
      let modifiedCnt = 0
      Object.values(this.files).forEach((file) => {
        if (regexp.test(file.relativePath)) {
          keys.forEach((key) => {
            if (key in file.frontmatter) {
              delete file.frontmatter[key]
              file.modified = true
              modifiedCnt++
            }
          })
        }
      })

      return modifiedCnt
    },
    normalizeFrontmatter(sourceKey: string[], targetKey: string, regexp: RegExp) {
      let modifiedCnt = 0
      Object.values(this.files).forEach((file) => {
        if (regexp.test(file.relativePath)) {
          sourceKey.forEach((key) => {
            if (key in file.frontmatter && key !== targetKey) {
              if (file.frontmatter[targetKey] === undefined) {
                file.frontmatter[targetKey] = file.frontmatter[key]
              }
              delete file.frontmatter[key]
            }
          })
          modifiedCnt++

          if (file.frontmatter[targetKey] === undefined) {
            file.frontmatter[targetKey] = ''
          }
        }
      })

      return modifiedCnt
    },
  },
})
