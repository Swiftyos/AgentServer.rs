-- CreateEnum
CREATE TYPE "UserGroupRole" AS ENUM ('MEMBER', 'OWNER');

-- CreateEnum
CREATE TYPE "AgentExecutionStatus" AS ENUM ('INCOMPLETE', 'QUEUED', 'RUNNING', 'COMPLETED', 'FAILED');

-- CreateEnum
CREATE TYPE "ExecutionTriggerType" AS ENUM ('MANUAL', 'SCHEDULE', 'WEBHOOK');

-- CreateEnum
CREATE TYPE "HttpMethod" AS ENUM ('GET', 'POST', 'PUT', 'DELETE', 'PATCH');

-- CreateEnum
CREATE TYPE "UserBlockCreditType" AS ENUM ('TOP_UP', 'USAGE');

-- CreateEnum
CREATE TYPE "SubmissionStatus" AS ENUM ('DAFT', 'PENDING', 'APPROVED', 'REJECTED');

-- CreateTable
CREATE TABLE "User" (
    "id" UUID NOT NULL,
    "email" TEXT NOT NULL,
    "name" TEXT,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "metadata" JSONB DEFAULT '{}',

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "UserGroup" (
    "id" UUID NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "groupIconUrl" TEXT,

    CONSTRAINT "UserGroup_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "UserGroupMembership" (
    "id" UUID NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "userId" UUID NOT NULL,
    "userGroupId" UUID NOT NULL,
    "Role" "UserGroupRole" NOT NULL DEFAULT 'MEMBER',

    CONSTRAINT "UserGroupMembership_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Agent" (
    "id" UUID NOT NULL,
    "version" INTEGER NOT NULL DEFAULT 1,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" TEXT,
    "description" TEXT,
    "createdByUserId" UUID,
    "groupId" UUID,
    "agentParentId" UUID,
    "agentParentVersion" INTEGER,

    CONSTRAINT "Agent_pkey" PRIMARY KEY ("id","version")
);

-- CreateTable
CREATE TABLE "AgentPreset" (
    "id" UUID NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "isActive" BOOLEAN NOT NULL DEFAULT true,
    "userId" UUID NOT NULL,
    "agentId" UUID NOT NULL,
    "agentVersion" INTEGER NOT NULL,

    CONSTRAINT "AgentPreset_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "UserAgent" (
    "id" UUID NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "userId" UUID NOT NULL,
    "agentId" UUID NOT NULL,
    "agentVersion" INTEGER NOT NULL,
    "agentPresetId" UUID,
    "isFavorite" BOOLEAN NOT NULL DEFAULT false,
    "isCreatedByUser" BOOLEAN NOT NULL DEFAULT false,
    "isArchived" BOOLEAN NOT NULL DEFAULT false,
    "isDeleted" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "UserAgent_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "AgentNode" (
    "id" UUID NOT NULL,
    "agentBlockId" UUID NOT NULL,
    "agentId" UUID NOT NULL,
    "agentVersion" INTEGER NOT NULL DEFAULT 1,
    "constantInput" JSONB NOT NULL DEFAULT '{}',
    "metadata" JSONB NOT NULL DEFAULT '{}',

    CONSTRAINT "AgentNode_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "AgentNodeLink" (
    "id" UUID NOT NULL,
    "agentNodeSourceId" UUID NOT NULL,
    "sourceName" TEXT NOT NULL,
    "agentNodeSinkId" UUID NOT NULL,
    "sinkName" TEXT NOT NULL,
    "isStatic" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "AgentNodeLink_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "AgentBlock" (
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "inputSchema" JSONB NOT NULL DEFAULT '{}',
    "outputSchema" JSONB NOT NULL DEFAULT '{}',

    CONSTRAINT "AgentBlock_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "AgentExecution" (
    "id" UUID NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "startedAt" TIMESTAMP(3),
    "executionTriggerType" "ExecutionTriggerType" NOT NULL DEFAULT 'MANUAL',
    "executionStatus" "AgentExecutionStatus" NOT NULL DEFAULT 'COMPLETED',
    "agentId" UUID NOT NULL,
    "agentVersion" INTEGER NOT NULL DEFAULT 1,
    "agentPresetId" UUID,
    "executedByUserId" UUID NOT NULL,
    "stats" JSONB NOT NULL DEFAULT '{}',

    CONSTRAINT "AgentExecution_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "AgentNodeExecution" (
    "id" UUID NOT NULL,
    "agentExecutionId" UUID NOT NULL,
    "agentNodeId" UUID NOT NULL,
    "executionStatus" "AgentExecutionStatus" NOT NULL DEFAULT 'COMPLETED',
    "executionData" TEXT,
    "addedTime" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "queuedTime" TIMESTAMP(3),
    "startedTime" TIMESTAMP(3),
    "endedTime" TIMESTAMP(3),
    "stats" JSONB NOT NULL DEFAULT '{}',

    CONSTRAINT "AgentNodeExecution_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "AgentNodeExecutionInputOutput" (
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "data" TEXT NOT NULL,
    "time" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "referencedByInputExecId" UUID,
    "referencedByOutputExecId" UUID,
    "agentPresetId" UUID,

    CONSTRAINT "AgentNodeExecutionInputOutput_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "AgentExecutionSchedule" (
    "id" UUID NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "agentPresetId" UUID NOT NULL,
    "schedule" TEXT NOT NULL,
    "isEnabled" BOOLEAN NOT NULL DEFAULT true,
    "triggerIdentifier" TEXT NOT NULL,
    "lastUpdated" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "userId" UUID NOT NULL,
    "agentId" UUID,
    "agentVersion" INTEGER,

    CONSTRAINT "AgentExecutionSchedule_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "WebhookTrigger" (
    "id" UUID NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "agentPresetId" UUID NOT NULL,
    "method" "HttpMethod" NOT NULL,
    "urlSlug" TEXT NOT NULL,
    "triggerIdentifier" TEXT NOT NULL,
    "isActive" BOOLEAN NOT NULL DEFAULT true,
    "lastReceivedDataAt" TIMESTAMP(3),
    "isDeleted" BOOLEAN NOT NULL DEFAULT false,
    "agentId" UUID,
    "agentVersion" INTEGER,

    CONSTRAINT "WebhookTrigger_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "AnalyticsDetails" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "userId" UUID NOT NULL,
    "type" TEXT NOT NULL,
    "data" JSONB NOT NULL DEFAULT '{}',
    "dataIndex" TEXT,

    CONSTRAINT "AnalyticsDetails_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "AnalyticsMetrics" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "analyticMetric" TEXT NOT NULL,
    "value" DOUBLE PRECISION NOT NULL,
    "dataString" TEXT,
    "userId" UUID NOT NULL,

    CONSTRAINT "AnalyticsMetrics_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "UserBlockCredit" (
    "transactionKey" TEXT NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "userId" UUID NOT NULL,
    "blockId" UUID,
    "executedAgentId" UUID,
    "executedAgentVersion" INTEGER,
    "agentNodeExecutionId" UUID,
    "amount" INTEGER NOT NULL,
    "type" "UserBlockCreditType" NOT NULL,
    "isActive" BOOLEAN NOT NULL DEFAULT true,
    "metadata" JSONB NOT NULL DEFAULT '{}',

    CONSTRAINT "UserBlockCredit_pkey" PRIMARY KEY ("transactionKey","userId")
);

-- CreateTable
CREATE TABLE "Profile" (
    "id" UUID NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "userId" UUID,
    "isGroupProfile" BOOLEAN NOT NULL DEFAULT false,
    "groupId" UUID,
    "username" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "links" TEXT[],
    "avatarUrl" TEXT,

    CONSTRAINT "Profile_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "StoreListing" (
    "id" UUID NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "isDeleted" BOOLEAN NOT NULL DEFAULT false,
    "isApproved" BOOLEAN NOT NULL DEFAULT false,
    "slug" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "mediaUrls" TEXT[],
    "agentId" UUID NOT NULL,
    "agentVersion" INTEGER NOT NULL,
    "owningUserId" UUID NOT NULL,
    "isGroupListing" BOOLEAN NOT NULL DEFAULT false,
    "owningGroupId" UUID,

    CONSTRAINT "StoreListing_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "StoreListingReview" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "storeListingId" UUID NOT NULL,
    "reviewByUserId" UUID NOT NULL,
    "score" INTEGER NOT NULL,
    "comments" TEXT,

    CONSTRAINT "StoreListingReview_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "StoreListingVersion" (
    "id" UUID NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "agentId" UUID NOT NULL,
    "agentVersion" INTEGER NOT NULL,
    "isFeatured" BOOLEAN NOT NULL DEFAULT false,
    "categories" TEXT[],
    "isDeleted" BOOLEAN NOT NULL DEFAULT false,
    "isAvailable" BOOLEAN NOT NULL DEFAULT true,
    "isApproved" BOOLEAN NOT NULL DEFAULT false,
    "storeListingId" UUID,

    CONSTRAINT "StoreListingVersion_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "StoreListingSubmission" (
    "id" UUID NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "storeListingId" UUID NOT NULL,
    "storeListingVersionId" UUID NOT NULL,
    "reviewByUserId" UUID NOT NULL,
    "Status" "SubmissionStatus" NOT NULL DEFAULT 'PENDING',
    "reviewComments" TEXT,

    CONSTRAINT "StoreListingSubmission_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "User_email_key" ON "User"("email");

-- CreateIndex
CREATE INDEX "UserGroup_name_idx" ON "UserGroup"("name");

-- CreateIndex
CREATE INDEX "UserGroupMembership_userId_idx" ON "UserGroupMembership"("userId");

-- CreateIndex
CREATE INDEX "UserGroupMembership_userGroupId_idx" ON "UserGroupMembership"("userGroupId");

-- CreateIndex
CREATE UNIQUE INDEX "UserGroupMembership_userId_userGroupId_key" ON "UserGroupMembership"("userId", "userGroupId");

-- CreateIndex
CREATE INDEX "AgentPreset_userId_idx" ON "AgentPreset"("userId");

-- CreateIndex
CREATE INDEX "UserAgent_userId_idx" ON "UserAgent"("userId");

-- CreateIndex
CREATE UNIQUE INDEX "AgentBlock_name_key" ON "AgentBlock"("name");

-- CreateIndex
CREATE UNIQUE INDEX "AgentNodeExecutionInputOutput_referencedByInputExecId_refer_key" ON "AgentNodeExecutionInputOutput"("referencedByInputExecId", "referencedByOutputExecId", "name");

-- CreateIndex
CREATE INDEX "AgentExecutionSchedule_isEnabled_idx" ON "AgentExecutionSchedule"("isEnabled");

-- CreateIndex
CREATE INDEX "WebhookTrigger_agentPresetId_idx" ON "WebhookTrigger"("agentPresetId");

-- CreateIndex
CREATE INDEX "analyticsDetails" ON "AnalyticsDetails"("userId", "type");

-- CreateIndex
CREATE INDEX "AnalyticsDetails_type_idx" ON "AnalyticsDetails"("type");

-- CreateIndex
CREATE UNIQUE INDEX "Profile_username_key" ON "Profile"("username");

-- CreateIndex
CREATE INDEX "Profile_username_idx" ON "Profile"("username");

-- CreateIndex
CREATE INDEX "StoreListing_isApproved_idx" ON "StoreListing"("isApproved");

-- CreateIndex
CREATE INDEX "StoreListing_agentId_idx" ON "StoreListing"("agentId");

-- CreateIndex
CREATE INDEX "StoreListing_owningUserId_idx" ON "StoreListing"("owningUserId");

-- CreateIndex
CREATE INDEX "StoreListing_owningGroupId_idx" ON "StoreListing"("owningGroupId");

-- CreateIndex
CREATE INDEX "StoreListingVersion_agentId_agentVersion_isApproved_idx" ON "StoreListingVersion"("agentId", "agentVersion", "isApproved");

-- CreateIndex
CREATE INDEX "StoreListingSubmission_storeListingId_idx" ON "StoreListingSubmission"("storeListingId");

-- CreateIndex
CREATE INDEX "StoreListingSubmission_Status_idx" ON "StoreListingSubmission"("Status");

-- AddForeignKey
ALTER TABLE "UserGroupMembership" ADD CONSTRAINT "UserGroupMembership_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserGroupMembership" ADD CONSTRAINT "UserGroupMembership_userGroupId_fkey" FOREIGN KEY ("userGroupId") REFERENCES "UserGroup"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Agent" ADD CONSTRAINT "Agent_createdByUserId_fkey" FOREIGN KEY ("createdByUserId") REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Agent" ADD CONSTRAINT "Agent_groupId_fkey" FOREIGN KEY ("groupId") REFERENCES "UserGroup"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Agent" ADD CONSTRAINT "Agent_agentParentId_agentParentVersion_fkey" FOREIGN KEY ("agentParentId", "agentParentVersion") REFERENCES "Agent"("id", "version") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentPreset" ADD CONSTRAINT "AgentPreset_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentPreset" ADD CONSTRAINT "AgentPreset_agentId_agentVersion_fkey" FOREIGN KEY ("agentId", "agentVersion") REFERENCES "Agent"("id", "version") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserAgent" ADD CONSTRAINT "UserAgent_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserAgent" ADD CONSTRAINT "UserAgent_agentId_agentVersion_fkey" FOREIGN KEY ("agentId", "agentVersion") REFERENCES "Agent"("id", "version") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserAgent" ADD CONSTRAINT "UserAgent_agentPresetId_fkey" FOREIGN KEY ("agentPresetId") REFERENCES "AgentPreset"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentNode" ADD CONSTRAINT "AgentNode_agentBlockId_fkey" FOREIGN KEY ("agentBlockId") REFERENCES "AgentBlock"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentNode" ADD CONSTRAINT "AgentNode_agentId_agentVersion_fkey" FOREIGN KEY ("agentId", "agentVersion") REFERENCES "Agent"("id", "version") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentNodeLink" ADD CONSTRAINT "AgentNodeLink_agentNodeSourceId_fkey" FOREIGN KEY ("agentNodeSourceId") REFERENCES "AgentNode"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentNodeLink" ADD CONSTRAINT "AgentNodeLink_agentNodeSinkId_fkey" FOREIGN KEY ("agentNodeSinkId") REFERENCES "AgentNode"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentExecution" ADD CONSTRAINT "AgentExecution_agentId_agentVersion_fkey" FOREIGN KEY ("agentId", "agentVersion") REFERENCES "Agent"("id", "version") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentExecution" ADD CONSTRAINT "AgentExecution_agentPresetId_fkey" FOREIGN KEY ("agentPresetId") REFERENCES "AgentPreset"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentExecution" ADD CONSTRAINT "AgentExecution_executedByUserId_fkey" FOREIGN KEY ("executedByUserId") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentNodeExecution" ADD CONSTRAINT "AgentNodeExecution_agentExecutionId_fkey" FOREIGN KEY ("agentExecutionId") REFERENCES "AgentExecution"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentNodeExecution" ADD CONSTRAINT "AgentNodeExecution_agentNodeId_fkey" FOREIGN KEY ("agentNodeId") REFERENCES "AgentNode"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentNodeExecutionInputOutput" ADD CONSTRAINT "AgentNodeExecutionInputOutput_referencedByInputExecId_fkey" FOREIGN KEY ("referencedByInputExecId") REFERENCES "AgentNodeExecution"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentNodeExecutionInputOutput" ADD CONSTRAINT "AgentNodeExecutionInputOutput_referencedByOutputExecId_fkey" FOREIGN KEY ("referencedByOutputExecId") REFERENCES "AgentNodeExecution"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentNodeExecutionInputOutput" ADD CONSTRAINT "AgentNodeExecutionInputOutput_agentPresetId_fkey" FOREIGN KEY ("agentPresetId") REFERENCES "AgentPreset"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentExecutionSchedule" ADD CONSTRAINT "AgentExecutionSchedule_agentPresetId_fkey" FOREIGN KEY ("agentPresetId") REFERENCES "AgentPreset"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentExecutionSchedule" ADD CONSTRAINT "AgentExecutionSchedule_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AgentExecutionSchedule" ADD CONSTRAINT "AgentExecutionSchedule_agentId_agentVersion_fkey" FOREIGN KEY ("agentId", "agentVersion") REFERENCES "Agent"("id", "version") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "WebhookTrigger" ADD CONSTRAINT "WebhookTrigger_agentPresetId_fkey" FOREIGN KEY ("agentPresetId") REFERENCES "AgentPreset"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "WebhookTrigger" ADD CONSTRAINT "WebhookTrigger_agentId_agentVersion_fkey" FOREIGN KEY ("agentId", "agentVersion") REFERENCES "Agent"("id", "version") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AnalyticsDetails" ADD CONSTRAINT "AnalyticsDetails_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "AnalyticsMetrics" ADD CONSTRAINT "AnalyticsMetrics_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserBlockCredit" ADD CONSTRAINT "UserBlockCredit_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserBlockCredit" ADD CONSTRAINT "UserBlockCredit_blockId_fkey" FOREIGN KEY ("blockId") REFERENCES "AgentBlock"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserBlockCredit" ADD CONSTRAINT "UserBlockCredit_executedAgentId_executedAgentVersion_fkey" FOREIGN KEY ("executedAgentId", "executedAgentVersion") REFERENCES "Agent"("id", "version") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserBlockCredit" ADD CONSTRAINT "UserBlockCredit_agentNodeExecutionId_fkey" FOREIGN KEY ("agentNodeExecutionId") REFERENCES "AgentNodeExecution"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Profile" ADD CONSTRAINT "Profile_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Profile" ADD CONSTRAINT "Profile_groupId_fkey" FOREIGN KEY ("groupId") REFERENCES "UserGroup"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StoreListing" ADD CONSTRAINT "StoreListing_agentId_agentVersion_fkey" FOREIGN KEY ("agentId", "agentVersion") REFERENCES "Agent"("id", "version") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StoreListing" ADD CONSTRAINT "StoreListing_owningUserId_fkey" FOREIGN KEY ("owningUserId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StoreListing" ADD CONSTRAINT "StoreListing_owningGroupId_fkey" FOREIGN KEY ("owningGroupId") REFERENCES "UserGroup"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StoreListingReview" ADD CONSTRAINT "StoreListingReview_storeListingId_fkey" FOREIGN KEY ("storeListingId") REFERENCES "StoreListing"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StoreListingReview" ADD CONSTRAINT "StoreListingReview_reviewByUserId_fkey" FOREIGN KEY ("reviewByUserId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StoreListingVersion" ADD CONSTRAINT "StoreListingVersion_agentId_agentVersion_fkey" FOREIGN KEY ("agentId", "agentVersion") REFERENCES "Agent"("id", "version") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StoreListingVersion" ADD CONSTRAINT "StoreListingVersion_storeListingId_fkey" FOREIGN KEY ("storeListingId") REFERENCES "StoreListing"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StoreListingSubmission" ADD CONSTRAINT "StoreListingSubmission_storeListingId_fkey" FOREIGN KEY ("storeListingId") REFERENCES "StoreListing"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StoreListingSubmission" ADD CONSTRAINT "StoreListingSubmission_storeListingVersionId_fkey" FOREIGN KEY ("storeListingVersionId") REFERENCES "StoreListingVersion"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StoreListingSubmission" ADD CONSTRAINT "StoreListingSubmission_reviewByUserId_fkey" FOREIGN KEY ("reviewByUserId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
