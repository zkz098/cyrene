export interface TreeNode {
  name: string
  type: 'folder' | 'file'
  path?: string // 添加完整路径字段，对于文件节点存储相对路径
  children?: TreeNode[]
}

function sortTreeNodes(nodes: TreeNode[]): TreeNode[] {
  return nodes.sort((a, b) => {
    // 文件在前，文件夹在后
    if (a.type === 'file' && b.type === 'folder')
      return -1
    if (a.type === 'folder' && b.type === 'file')
      return 1
    // 同类型按名称排序
    return a.name.localeCompare(b.name)
  }).map((node) => {
    // 递归排序子节点
    if (node.children) {
      node.children = sortTreeNodes(node.children)
    }
    return node
  })
}

export function buildFileTree(paths: string[]): TreeNode[] {
  const root: TreeNode[] = []

  for (const path of paths) {
    const parts = path.split('/').filter(Boolean)
    let currentLevel = root
    let currentPath = ''

    for (let i = 0; i < parts.length; i++) {
      const part = parts[i]
      const isFile = i === parts.length - 1
      currentPath += (currentPath ? '/' : '') + part

      let existingNode = currentLevel.find(node => node.name === part)

      if (!existingNode) {
        existingNode = {
          name: part,
          type: isFile ? 'file' : 'folder',
          path: isFile ? path : undefined, // 只为文件节点保存完整路径
          ...(isFile ? {} : { children: [] }),
        } as TreeNode

        currentLevel.push(existingNode)
      }

      if (!isFile && existingNode.children) {
        currentLevel = existingNode.children
      }
    }
  }

  // 对整个树进行排序：文件在前，文件夹在后
  return sortTreeNodes(root)
}
