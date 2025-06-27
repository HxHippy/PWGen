use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use zeroize::Zeroize;

use crate::{Result, Error};
use crate::secrets::DecryptedSecretEntry;
use crate::crypto::MasterKey;

/// Permission levels for team members
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    /// Can only view secrets
    Read,
    /// Can view and edit secrets
    Write,
    /// Can view, edit, and share secrets with others
    Share,
    /// Full administrative access
    Admin,
}

/// Team member information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub id: String,
    pub email: String,
    pub name: String,
    pub public_key: Vec<u8>,
    pub role: Permission,
    pub added_at: DateTime<Utc>,
    pub last_activity: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// Team information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub members: Vec<TeamMember>,
    pub is_active: bool,
}

/// Shared secret information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSecret {
    pub id: String,
    pub secret_id: String,
    pub team_id: String,
    pub shared_by: String,
    pub shared_at: DateTime<Utc>,
    pub permissions: Permission,
    pub encrypted_secret_key: Vec<u8>, // Secret key encrypted with team's public key
    pub expiration: Option<DateTime<Utc>>,
    pub access_count: u64,
    pub last_accessed: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// Share request for sharing secrets with team members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareRequest {
    pub id: String,
    pub secret_id: String,
    pub requested_by: String,
    pub requested_from: String,
    pub team_id: Option<String>,
    pub permissions: Permission,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: ShareRequestStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShareRequestStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
}

/// Access log entry for audit purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessLog {
    pub id: String,
    pub secret_id: String,
    pub user_id: String,
    pub team_id: Option<String>,
    pub action: AccessAction,
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: bool,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessAction {
    View,
    Edit,
    Share,
    Delete,
    Download,
    Copy,
}

/// Team sharing and access control manager
pub struct TeamSharingManager;

impl TeamSharingManager {
    /// Create a new team
    pub fn create_team(
        name: String,
        description: Option<String>,
        owner_id: String,
        owner_email: String,
        owner_name: String,
        owner_public_key: Vec<u8>,
    ) -> Result<Team> {
        let team_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let owner = TeamMember {
            id: owner_id.clone(),
            email: owner_email,
            name: owner_name,
            public_key: owner_public_key,
            role: Permission::Admin,
            added_at: now,
            last_activity: Some(now),
            is_active: true,
        };

        Ok(Team {
            id: team_id,
            name,
            description,
            owner_id,
            created_at: now,
            updated_at: now,
            members: vec![owner],
            is_active: true,
        })
    }

    /// Add a member to a team
    pub fn add_team_member(
        team: &mut Team,
        member_id: String,
        email: String,
        name: String,
        public_key: Vec<u8>,
        role: Permission,
        added_by: &str,
    ) -> Result<()> {
        // Check if the person adding has admin rights
        let adder = team.members.iter()
            .find(|m| m.id == added_by)
            .ok_or_else(|| Error::Other("User not found in team".to_string()))?;
        
        if adder.role != Permission::Admin {
            return Err(Error::Other("Insufficient permissions to add members".to_string()));
        }

        // Check if member already exists
        if team.members.iter().any(|m| m.id == member_id || m.email == email) {
            return Err(Error::Other("Member already exists in team".to_string()));
        }

        let member = TeamMember {
            id: member_id,
            email,
            name,
            public_key,
            role,
            added_at: Utc::now(),
            last_activity: None,
            is_active: true,
        };

        team.members.push(member);
        team.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a member from a team
    pub fn remove_team_member(
        team: &mut Team,
        member_id: &str,
        removed_by: &str,
    ) -> Result<()> {
        // Check if the person removing has admin rights
        let remover = team.members.iter()
            .find(|m| m.id == removed_by)
            .ok_or_else(|| Error::Other("User not found in team".to_string()))?;
        
        if remover.role != Permission::Admin {
            return Err(Error::Other("Insufficient permissions to remove members".to_string()));
        }

        // Can't remove the team owner
        if member_id == team.owner_id {
            return Err(Error::Other("Cannot remove team owner".to_string()));
        }

        let member_index = team.members.iter()
            .position(|m| m.id == member_id)
            .ok_or_else(|| Error::Other("Member not found in team".to_string()))?;

        team.members.remove(member_index);
        team.updated_at = Utc::now();
        Ok(())
    }

    /// Update a member's role
    pub fn update_member_role(
        team: &mut Team,
        member_id: &str,
        new_role: Permission,
        updated_by: &str,
    ) -> Result<()> {
        // Check if the person updating has admin rights
        let updater = team.members.iter()
            .find(|m| m.id == updated_by)
            .ok_or_else(|| Error::Other("User not found in team".to_string()))?;
        
        if updater.role != Permission::Admin {
            return Err(Error::Other("Insufficient permissions to update roles".to_string()));
        }

        // Can't change the team owner's role
        if member_id == team.owner_id {
            return Err(Error::Other("Cannot change team owner's role".to_string()));
        }

        let member = team.members.iter_mut()
            .find(|m| m.id == member_id)
            .ok_or_else(|| Error::Other("Member not found in team".to_string()))?;

        member.role = new_role;
        team.updated_at = Utc::now();
        Ok(())
    }

    /// Share a secret with a team
    pub fn share_secret_with_team(
        secret: &DecryptedSecretEntry,
        team: &Team,
        shared_by: &str,
        permissions: Permission,
        expiration: Option<DateTime<Utc>>,
        secret_key: &[u8],
    ) -> Result<SharedSecret> {
        // Check if the person sharing has the right to share
        let sharer = team.members.iter()
            .find(|m| m.id == shared_by)
            .ok_or_else(|| Error::Other("User not found in team".to_string()))?;
        
        if sharer.role != Permission::Share && sharer.role != Permission::Admin {
            return Err(Error::Other("Insufficient permissions to share secrets".to_string()));
        }

        // For now, we'll use a simple approach - in a real implementation,
        // you'd use proper public key cryptography to encrypt the secret key
        // Using a dummy encryption for demonstration purposes
        let master_key = MasterKey::derive_from_password("dummy", &[0u8; 32])?;
        let encrypted_secret_key = master_key.encrypt(secret_key)?;

        Ok(SharedSecret {
            id: Uuid::new_v4().to_string(),
            secret_id: secret.id.clone(),
            team_id: team.id.clone(),
            shared_by: shared_by.to_string(),
            shared_at: Utc::now(),
            permissions,
            encrypted_secret_key,
            expiration,
            access_count: 0,
            last_accessed: None,
            is_active: true,
        })
    }

    /// Create a share request
    pub fn create_share_request(
        secret_id: String,
        requested_by: String,
        requested_from: String,
        team_id: Option<String>,
        permissions: Permission,
        message: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ShareRequest> {
        Ok(ShareRequest {
            id: Uuid::new_v4().to_string(),
            secret_id,
            requested_by,
            requested_from,
            team_id,
            permissions,
            message,
            created_at: Utc::now(),
            expires_at,
            status: ShareRequestStatus::Pending,
        })
    }

    /// Approve a share request
    pub fn approve_share_request(
        request: &mut ShareRequest,
        approved_by: &str,
    ) -> Result<()> {
        if request.requested_from != approved_by {
            return Err(Error::Other("Only the secret owner can approve share requests".to_string()));
        }

        request.status = ShareRequestStatus::Approved;
        Ok(())
    }

    /// Reject a share request
    pub fn reject_share_request(
        request: &mut ShareRequest,
        rejected_by: &str,
    ) -> Result<()> {
        if request.requested_from != rejected_by {
            return Err(Error::Other("Only the secret owner can reject share requests".to_string()));
        }

        request.status = ShareRequestStatus::Rejected;
        Ok(())
    }

    /// Check if a user has access to a secret
    pub fn check_secret_access(
        user_id: &str,
        secret_id: &str,
        shared_secrets: &[SharedSecret],
        teams: &[Team],
        required_permission: &Permission,
    ) -> Result<bool> {
        // Find shared secret
        let shared_secret = shared_secrets.iter()
            .find(|s| s.secret_id == secret_id && s.is_active)
            .ok_or_else(|| Error::Other("Secret not shared".to_string()))?;

        // Check if expired
        if let Some(expiration) = shared_secret.expiration {
            if Utc::now() > expiration {
                return Ok(false);
            }
        }

        // Find team
        let team = teams.iter()
            .find(|t| t.id == shared_secret.team_id)
            .ok_or_else(|| Error::Other("Team not found".to_string()))?;

        // Check if user is in team
        let member = team.members.iter()
            .find(|m| m.id == user_id && m.is_active)
            .ok_or_else(|| Error::Other("User not in team".to_string()))?;

        // Check permissions
        let has_permission = match required_permission {
            Permission::Read => true, // All team members can read
            Permission::Write => {
                matches!(member.role, Permission::Write | Permission::Share | Permission::Admin)
            }
            Permission::Share => {
                matches!(member.role, Permission::Share | Permission::Admin)
            }
            Permission::Admin => matches!(member.role, Permission::Admin),
        };

        Ok(has_permission && 
           Self::permission_allows(&shared_secret.permissions, required_permission))
    }

    /// Check if a permission level allows a required permission
    fn permission_allows(granted: &Permission, required: &Permission) -> bool {
        match (granted, required) {
            (Permission::Admin, _) => true,
            (Permission::Share, Permission::Share) => true,
            (Permission::Share, Permission::Write) => true,
            (Permission::Share, Permission::Read) => true,
            (Permission::Write, Permission::Write) => true,
            (Permission::Write, Permission::Read) => true,
            (Permission::Read, Permission::Read) => true,
            _ => false,
        }
    }

    /// Log access to a secret
    pub fn log_access(
        secret_id: String,
        user_id: String,
        team_id: Option<String>,
        action: AccessAction,
        success: bool,
        ip_address: Option<String>,
        user_agent: Option<String>,
        details: Option<String>,
    ) -> AccessLog {
        AccessLog {
            id: Uuid::new_v4().to_string(),
            secret_id,
            user_id,
            team_id,
            action,
            timestamp: Utc::now(),
            ip_address,
            user_agent,
            success,
            details,
        }
    }

    /// Update access count for a shared secret
    pub fn update_access_count(shared_secret: &mut SharedSecret) {
        shared_secret.access_count += 1;
        shared_secret.last_accessed = Some(Utc::now());
    }

    /// Get team members with specific permission
    pub fn get_team_members_with_permission<'a>(
        team: &'a Team,
        permission: &Permission,
    ) -> Vec<&'a TeamMember> {
        team.members.iter()
            .filter(|m| m.is_active && Self::permission_allows(&m.role, permission))
            .collect()
    }

    /// Get shared secrets for a user
    pub fn get_user_shared_secrets(
        user_id: &str,
        shared_secrets: &[SharedSecret],
        teams: &[Team],
    ) -> Result<Vec<SharedSecret>> {
        let mut user_secrets = Vec::new();

        for shared_secret in shared_secrets {
            if !shared_secret.is_active {
                continue;
            }

            // Check if expired
            if let Some(expiration) = shared_secret.expiration {
                if Utc::now() > expiration {
                    continue;
                }
            }

            // Find team
            if let Some(team) = teams.iter().find(|t| t.id == shared_secret.team_id) {
                // Check if user is in team
                if team.members.iter().any(|m| m.id == user_id && m.is_active) {
                    user_secrets.push(shared_secret.clone());
                }
            }
        }

        Ok(user_secrets)
    }

    /// Revoke access to a shared secret
    pub fn revoke_shared_secret(
        shared_secret: &mut SharedSecret,
        revoked_by: &str,
        teams: &[Team],
    ) -> Result<()> {
        // Find team
        let team = teams.iter()
            .find(|t| t.id == shared_secret.team_id)
            .ok_or_else(|| Error::Other("Team not found".to_string()))?;

        // Check if the person revoking has admin rights or is the original sharer
        let revoker = team.members.iter()
            .find(|m| m.id == revoked_by)
            .ok_or_else(|| Error::Other("User not found in team".to_string()))?;
        
        if revoker.role != Permission::Admin && shared_secret.shared_by != revoked_by {
            return Err(Error::Other("Insufficient permissions to revoke access".to_string()));
        }

        shared_secret.is_active = false;
        Ok(())
    }
}

/// Secure zeroization for sensitive data
impl Drop for TeamMember {
    fn drop(&mut self) {
        self.public_key.zeroize();
    }
}

impl Drop for SharedSecret {
    fn drop(&mut self) {
        self.encrypted_secret_key.zeroize();
    }
}