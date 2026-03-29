terraform {
  required_providers {
    aws = { source = "hashicorp/aws", version = "~> 5.0" }
  }
}

resource "aws_ecs_cluster" "todo" {
  name = "todo-cluster"
}

resource "aws_ecs_service" "api" {
  name            = "todo-api"
  cluster         = aws_ecs_cluster.todo.id
  task_definition = aws_ecs_task_definition.api.arn
  desired_count   = 3
}
