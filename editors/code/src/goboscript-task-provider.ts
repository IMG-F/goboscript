import * as vscode from "vscode"
import * as fs from "fs"
import * as path from "path"

interface GoboscriptTaskDefinition extends vscode.TaskDefinition {
  env: string
}

export class GoboscriptTaskProvider implements vscode.TaskProvider {
  static GoboscriptType = "goboscript-build"
  private workspaceRoot: vscode.WorkspaceFolder

  constructor(workspaceRoot: vscode.WorkspaceFolder) {
    this.workspaceRoot = workspaceRoot
  }

  provideTasks(
    token: vscode.CancellationToken
  ): vscode.ProviderResult<vscode.Task[]> {
    const rootPath = this.workspaceRoot.uri.fsPath
    const files = fs.readdirSync(rootPath)
    const tasks: vscode.Task[] = []

    for (const file of files) {
      if (file.startsWith("goboscript") && file.endsWith(".toml")) {
        let envName = ""
        const match = file.match(/^goboscript(?:-(.+))?\.toml$/)
        if (match && match[1]) {
          envName = match[1]
        }
        
        let command = envName === "" ? `goboscript build` : `goboscript build -e ${envName}`
        let definition: GoboscriptTaskDefinition = { type: GoboscriptTaskProvider.GoboscriptType, env: envName }

        const task = new vscode.Task(
          definition,
          this.workspaceRoot,
          `build ${envName ? "(" + envName + ")" : ""}`,
          "goboscript",
          new vscode.ShellExecution(command),
          ["goboscript"]
        )

        tasks.push(task)
      }
    }

    return tasks
  }

  resolveTask(
    task: vscode.Task,
    token: vscode.CancellationToken,
  ): vscode.ProviderResult<vscode.Task> {
    let envName = (task.definition as GoboscriptTaskDefinition).env

    if (!envName && task.name) {
      const match = task.name.match(/\(([^)]+)\)/)
      if (match && match[1]) envName = match[1]
    }

    envName = envName || ""

    let command = !envName ? `goboscript build` : `goboscript build -e ${envName}`

    return new vscode.Task(
      task.definition,
      this.workspaceRoot,
      `build ${envName ? "(" + envName + ")" : ""}`,
      "goboscript",
      new vscode.ShellExecution(command),
      ["goboscript"]
    )
  }
}