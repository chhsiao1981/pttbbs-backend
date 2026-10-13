
from typing import Literal

from pydantic import BaseModel, Field

from ..comment.types import CommentNotification
from ..community.types import (
    Community,
    CommunityFollowerView,
    CommunityModeratorView,
    CommunityView,
    ModActionNotification,
    MultiCommunityView,
)
from ..message.types import PrivateMessageNotification
from ..post.types import PostNotification
from ..site.types import Instance
from ..types import ID, ListParams, NotificationType, SensitiveString, Time3339
from ..user.types import LocalUser, LocalUserView, PersonView
from ..user.types.person import Person


class GenerateTOTPResult(BaseModel):
    totp_secret_url: str


class Account(BaseModel):
    discussion_languages: list[ID]
    keyword_blocks: list[str]
    person_blocks: list[Person]
    instance_persons_blocks: list[Instance]
    instance_communities_blocks: list[Instance]
    community_blocks: list[Community]
    multi_community_follows: list[MultiCommunityView]
    moderates: list[CommunityModeratorView]
    follows: list[CommunityFollowerView]
    local_user_view: LocalUserView


class DeleteAccountParams(BaseModel):
    delete_content: bool
    password: SensitiveString


class AccountImportParams(BaseModel):
    discussion_languages: list[str] = Field(description='json string of ID')
    blocking_keywords: list[str]
    blocked_instances_persons: list[str] = Field(
        description='json string of Instance')
    blocked_instances_communities: list[str] = Field(
        description='json string of Instance')
    blocked_users: list[str] = Field(description='json string of Person')
    blocked_communities: list[str] = Field(
        description='json string of Community')
    saved_comments: list[str] = Field(description='json string of Comment')
    saved_posts: list[str] = Field(description='json string of Post')
    followed_communities: list[str] = Field(
        description='json string of CommunityFollowView')
    settings: LocalUser | None
    bot_account: bool = False
    matrix_id: str
    banner: str
    avatar: str
    bio: str
    display_name: str


class LoginToken(BaseModel):
    user_agent: str = ''
    ip: str = ''
    published_at: Time3339
    user_id: ID


class ListLoginResult(BaseModel):
    logins: list[LoginToken]


class DeleteMediaParams(BaseModel):
    filename: str


class EditTOTPParams(BaseModel):
    enabled: bool
    totp_token: str


class EditTOTPResult(BaseModel):
    enabled: bool


class GetUnreadCountsResult(BaseModel):
    registration_application_count: int = 0
    pending_follow_count: int = 0
    report_count: int = 0
    notification_count: int


class BlockCommunityParams(BaseModel):
    block: bool
    community_id: ID


class BlockCommunityResult(BaseModel):
    discussion_languages: list[ID]
    community_view: CommunityView


class AuthRegisterParams(BaseModel):
    token: str = ''
    stay_logged_in: bool = False
    answer: str = ''
    honeypot: str = ''
    captcha_answer: str = ''
    captcha_uuid: str = ''
    email: SensitiveString
    show_nsfw: bool
    password_verify: SensitiveString
    password: SensitiveString
    username: str


class AuthResult(BaseModel):
    verify_email_sent: bool = Field(description='If email verifications are required, this will return true for a signup response.')  # noqa
    registration_created: bool = Field(description='If registration applications are required, this will return true for a signup response.')  # noqa
    jwt: SensitiveString


class AuthLoginParams(BaseModel):
    stay_logged_in: bool = False
    totp_2fa_token: str = ''
    password: str
    username_or_email: str


class BlockPersonParams(BaseModel):
    block: bool
    person_id: ID


class BlockPersonResult(BaseModel):
    person_view: PersonView


class CaptchaResponse(BaseModel):
    uuid: str
    wav: str = Field(description='A Base64 encoded wav audio')
    png: str = Field(description='A Base64 encoded png')


class GetCaptchaResult(BaseModel):
    ok: CaptchaResponse


class ResetPasswordParams(BaseModel):
    email: SensitiveString


class ChangePasswordEmailResetParams(BaseModel):
    password_verify: SensitiveString
    password: SensitiveString
    token: SensitiveString


class MarkNotificationAsReadParams(BaseModel):
    read: bool
    notification_id: ID


class ChangePasswordParams(BaseModel):
    stay_logged_in: bool = False
    old_password: SensitiveString
    new_password_verify: SensitiveString
    new_password: SensitiveString


class ListNotificationParams(ListParams):
    creator_id: ID
    unread_only: bool
    type_: NotificationType | Literal['all']


class NotificationComment(CommentNotification):
    type_: str = 'comment'


class NotificationPost(PostNotification):
    type_: str = 'post'


class NotificationPrivateMessage(PrivateMessageNotification):
    type_: str = 'private_message'


class NotificationModAction(ModActionNotification):
    type_: str = 'mod_action'


type NotificationData = NotificationComment | NotificationPost | NotificationPrivateMessage | \
    NotificationModAction


class Notification(BaseModel):
    creator_id: ID
    modlog_id: ID | None = None
    private_message_id: ID | None = None
    post_id: ID | None = None
    kind: NotificationType
    published_at: Time3339
    read: bool
    comment_id: ID | None = None
    recipient_id: ID
    id: ID


class NotificationView(BaseModel):
    data: NotificationData
    notification: Notification


class VerifyEmailParams(BaseModel):
    token: str


class ResendVerificationEmailParams(BaseModel):
    email: str
