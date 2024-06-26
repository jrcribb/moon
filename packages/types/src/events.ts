import type { Duration, Runtime } from './common';
import type { Action, ActionContext, ActionNode } from './pipeline';
import type { Project } from './project';

export interface ProviderEnvironment {
	baseBranch: string | null;
	branch: string;
	id: string;
	provider: string;
	requestId: string | null;
	requestUrl: string | null;
	revision: string;
	url: string | null;
}

export interface WebhookPayload<T extends EventType, E> {
	createdAt: string;
	environment: ProviderEnvironment | null;
	event: E;
	type: T;
	uuid: string;
}

export type EventType =
	| 'action.finished'
	| 'action.started'
	| 'dependencies.installed'
	| 'dependencies.installing'
	| 'pipeline.aborted'
	| 'pipeline.finished'
	| 'pipeline.started'
	| 'project.synced'
	| 'project.syncing'
	| 'target.ran'
	| 'target.running'
	| 'tool.installed'
	| 'tool.installing'
	| 'workspace.synced'
	| 'workspace.syncing';

export interface EventActionStarted {
	action: Action;
	node: ActionNode;
}

export type PayloadActionStarted = WebhookPayload<'action.started', EventActionStarted>;

export interface EventActionFinished {
	action: Action;
	error: string | null;
	node: ActionNode;
}

export type PayloadActionFinished = WebhookPayload<'action.finished', EventActionFinished>;

export interface EventDependenciesInstalling {
	project: Project | null;
	runtime: Runtime;
}

export type PayloadDependenciesInstalling = WebhookPayload<
	'dependencies.installing',
	EventDependenciesInstalling
>;

export interface EventDependenciesInstalled {
	error: string | null;
	project: Project | null;
	runtime: Runtime;
}

export type PayloadDependenciesInstalled = WebhookPayload<
	'dependencies.installed',
	EventDependenciesInstalled
>;

export interface EventProjectSyncing {
	project: Project;
	runtime: Runtime;
}

export type PayloadProjectSyncing = WebhookPayload<'project.syncing', EventProjectSyncing>;

export interface EventProjectSynced {
	error: string | null;
	project: Project;
	runtime: Runtime;
}

export type PayloadProjectSynced = WebhookPayload<'project.synced', EventProjectSynced>;

export interface EventPipelineAborted {
	error: string;
}

export type PayloadPipelineAborted = WebhookPayload<'pipeline.aborted', EventPipelineAborted>;

export interface EventPipelineStarted {
	actionsCount: number;
	context: ActionContext;
}

export type PayloadPipelineStarted = WebhookPayload<'pipeline.started', EventPipelineStarted>;

export interface EventPipelineFinished {
	baselineDuration: Duration;
	cachedCount: number;
	context: ActionContext;
	duration: Duration;
	estimatedSavings: Duration | null;
	failedCount: number;
	passedCount: number;
}

export type PayloadPipelineFinished = WebhookPayload<'pipeline.finished', EventPipelineFinished>;

export interface EventTargetRunning {
	target: string;
}

export type PayloadTargetRunning = WebhookPayload<'target.running', EventTargetRunning>;

export interface EventTargetRan {
	error: string | null;
	target: string;
}

export type PayloadTargetRan = WebhookPayload<'target.ran', EventTargetRan>;

export interface EventToolInstalling {
	runtime: Runtime;
}

export type PayloadToolInstalling = WebhookPayload<'tool.installing', EventToolInstalling>;

export interface EventToolInstalled {
	error: string | null;
	runtime: Runtime;
}

export type PayloadToolInstalled = WebhookPayload<'tool.installed', EventToolInstalled>;

export type PayloadWorkspaceSyncing = WebhookPayload<'workspace.syncing', {}>;

export interface EventWorkspaceSynced {
	error: string | null;
}

export type PayloadWorkspaceSynced = WebhookPayload<'workspace.synced', EventWorkspaceSynced>;
