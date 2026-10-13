from typing import Annotated

from fastapi import APIRouter, Query

from ..media.types import LocalImageView
from ..types import ListParams, ListResult, Result
from ..user.types import PartialLocalUser
from .types import (
    Account,
    AccountImportParams,
    AuthLoginParams,
    AuthRegisterParams,
    AuthResult,
    BlockCommunityParams,
    BlockCommunityResult,
    BlockPersonParams,
    BlockPersonResult,
    ChangePasswordEmailResetParams,
    ChangePasswordParams,
    DeleteAccountParams,
    DeleteMediaParams,
    EditTOTPParams,
    EditTOTPResult,
    GetCaptchaResult,
    GetUnreadCountsResult,
    ListNotificationParams,
    MarkNotificationAsReadParams,
    NotificationView,
    ResendVerificationEmailParams,
    ResetPasswordParams,
    VerifyEmailParams,
)

router = APIRouter(
    tags=['account']
)


@router.post('/account/auth/totp/generate')
def generate_totp():
    ...


@router.get('/account')
def get_account() -> Account:
    ...


@router.delete('/account')
def delete_account(params: DeleteAccountParams) -> Result:
    ...


@router.get('/account/settings/export')
def account_export() -> AccountImportParams:
    ...


@router.post('/account/settings/import')
def account_import(params: AccountImportParams) -> Result:
    ...


@router.get('/account/login/list')
def list_login() -> AccountImportParams:
    ...


@router.get('/account/validate_auth')
def validate_token() -> Result:
    ...


@router.get('/account/media_list')
def list_media(params: Annotated[ListParams, Query()]) -> ListResult[LocalImageView]:
    ...


@router.delete('/account/media')
def delete_media(params: DeleteMediaParams) -> Result:
    ...


@router.post('/account/auth/totp/edit')
def edit_totp(params: EditTOTPParams) -> EditTOTPResult:
    '''
    Enable / Disable TOTP / two-factor authentication. To enable, you need to first call `/account/auth/totp/generate` and then pass a valid token to this. Disabling is only possible if 2FA was previously enabled. Again it is necessary to pass a valid token.
    '''  # noqa
    ...


@router.get('/account/unread_counts')
def get_unread_counts() -> GetUnreadCountsResult:
    '''
    Returns the amount of unread items of various types. For normal users this means * the number of unread notifications, mods and admins get additional unread counts for reports, registration applications and pending follows to private communities.
    '''  # noqa
    ...


@router.post('/account/block/community', tags=['community'])
def block_community(params: BlockCommunityParams) -> BlockCommunityResult:
    ...


@router.post('/account/auth/register')
def auth_register(params: AuthRegisterParams) -> AuthResult:
    ...


@router.post('/account/auth/login')
def auth_login(params: AuthLoginParams) -> AuthResult:
    ...


@router.post('/account/auth/logout')
def auth_logout() -> Result:
    ...


@router.post('/account/block/person', tags=['person'])
def block_person(params: BlockPersonParams) -> BlockPersonResult:
    ...


@router.get('/account/auth/get_captcha')
def get_captcha() -> GetCaptchaResult:
    ...


@router.post('/account/auth/password_reset')
def reset_password(params: ResetPasswordParams) -> Result:
    ...


@router.post('/account/auth/password_reset')
def change_password_email_reset(params: ChangePasswordEmailResetParams) -> Result:
    ...


@router.post('/account/notification/mark_as_read/all')
def mark_all_notification_as_read() -> Result:
    ...


@router.post('/account/notification/mark_as_read')
def mark_notification_as_read(params: MarkNotificationAsReadParams) -> Result:
    ...


@router.post('/account/settings/save')
def account_save(params: PartialLocalUser) -> Result:
    ...


@router.post('/account/auth/change_password')
def change_password(params: ChangePasswordParams) -> AuthResult:
    ...


@router.post('/account/notification/list')
def list_notification(params: ListNotificationParams) -> ListResult[NotificationView]:
    ...


@router.post('/account/auth/verify_email')
def verify_email(params: VerifyEmailParams) -> Result:
    ...


@router.post('/account/auth/resend_verification_email')
def resend_verification_email(params: ResendVerificationEmailParams) -> Result:
    ...
